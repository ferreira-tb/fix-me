use crate::error::Result;
use crate::{bail, http};
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use std::sync::Arc;
use strum::Display;
use tauri::AppHandle;
use tauri_plugin_svelte::ManagerExt;
use tokio::sync::{OwnedSemaphorePermit, Semaphore};

pub struct Llm {
  app: AppHandle,
  semaphore: Arc<Semaphore>,
}

impl Llm {
  pub fn new(app: AppHandle) -> Self {
    Self {
      app,
      semaphore: Arc::new(Semaphore::new(1)),
    }
  }

  pub async fn prompt(&self, prompt: String) -> Result<String> {
    let settings = self
      .app
      .svelte()
      .try_state::<Settings>("settings")?;

    let Some(token) = settings.token.as_deref() else {
      bail!("unauthorized: missing token");
    };

    let permit = self.acquire_permit().await;
    let prompt = build_prompt(&settings, prompt);
    let response: Json = http::post(permit, token, &prompt).await?;

    let answer: Option<String> = try {
      response
        .as_object()?
        .get("choices")?
        .as_array()?
        .get(0)?
        .as_object()?
        .get("message")?
        .as_object()?
        .get("content")?
        .as_str()?
        .to_owned()
    };

    answer
      .ok_or_else(|| anyhow!("invalid response: {response:?}"))
      .map_err(Into::into)
  }

  async fn acquire_permit(&self) -> OwnedSemaphorePermit {
    let semaphore = Arc::clone(&self.semaphore);
    semaphore
      .acquire_owned()
      .await
      .expect("semaphore wouldn't be closed")
  }
}

fn build_prompt(settings: &Settings, prompt: String) -> Prompt {
  let content = String::from(
    "
    Analyze the following text and then modify it to meet the specified criteria.
    The criteria are as follows:
    ",
  );

  let mut dev_message = Message {
    role: Role::Developer,
    content: content.trim().into(),
  };

  if settings.grammar {
    dev_message
      .content
      .push_str("Fix any grammatical errors.");
  }

  if settings.readability {
    dev_message
      .content
      .push_str("Improve readability.");
  }

  if settings.tone {
    dev_message
      .content
      .push_str("Check the tone used.");
  }

  if settings.politeness {
    dev_message
      .content
      .push_str("Make sure the text is polite.");
  }

  if settings.formality {
    dev_message
      .content
      .push_str("Ensure the text is formal.");
  }

  dev_message
    .content
    .push_str("Your answer should contain only the modified text and nothing else.");

  Prompt::with_messages([
    dev_message,
    Message {
      role: Role::User,
      content: prompt.trim().into(),
    },
  ])
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Settings {
  formality: bool,
  grammar: bool,
  politeness: bool,
  readability: bool,
  token: Option<String>,
  tone: bool,
}

#[derive(Default, Serialize)]
struct Prompt {
  model: Model,
  messages: Vec<Message>,
}

impl Prompt {
  fn with_messages<I>(messages: I) -> Self
  where
    I: IntoIterator<Item = Message>,
  {
    Self {
      model: Model::default(),
      messages: messages.into_iter().collect(),
    }
  }
}

#[derive(Default, Display, Serialize)]
enum Model {
  #[default]
  #[serde(rename = "gpt-4o-mini")]
  #[strum(serialize = "gpt-4o-mini")]
  Gpt4Mini,
}

#[derive(Serialize)]
struct Message {
  role: Role,
  content: String,
}

#[derive(Default, Display, Serialize)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
enum Role {
  #[default]
  User,
  Developer,
}
