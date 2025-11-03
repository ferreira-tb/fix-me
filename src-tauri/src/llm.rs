use crate::http;
use anyhow::{Result, anyhow, bail};
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use specta::Type;
use std::sync::Arc;
use strum::Display;
use tauri::AppHandle;
use tauri_plugin_pinia::ManagerExt;
use tokio::sync::{OwnedSemaphorePermit, Semaphore};

pub const DEFAULT_FORMALITY: &str = "Ensure the text is formal.";
pub const DEFAULT_GRAMMAR: &str = "Fix any grammatical errors.";
pub const DEFAULT_POLITENESS: &str = "Make sure the text is polite.";
pub const DEFAULT_READABILITY: &str = "Improve readability.";
pub const DEFAULT_TONE: &str = "Check the tone used.";

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
      .pinia()
      .state::<Settings>("settings")?;

    let Some(token) = settings.token.as_deref() else {
      bail!("unauthorized: missing token");
    };

    let permit = self.acquire_permit().await;
    let prompt = build_prompt(&settings, &prompt);
    let response: Json = http::post(permit, token, &prompt).await?;

    let answer: Option<String> = try {
      response
        .as_object()?
        .get("choices")?
        .as_array()?
        .first()?
        .as_object()?
        .get("message")?
        .as_object()?
        .get("content")?
        .as_str()?
        .to_owned()
    };

    answer.ok_or_else(|| anyhow!("invalid response: {response:?}"))
  }

  async fn acquire_permit(&self) -> OwnedSemaphorePermit {
    let semaphore = Arc::clone(&self.semaphore);
    semaphore
      .acquire_owned()
      .await
      .expect("semaphore wouldn't be closed")
  }
}

fn build_prompt(settings: &Settings, prompt: &str) -> Prompt {
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

  if settings.grammar.enabled {
    dev_message
      .content
      .push_str(&settings.grammar.message);
  }

  if settings.readability.enabled {
    dev_message
      .content
      .push_str(&settings.readability.message);
  }

  if settings.tone.enabled {
    dev_message
      .content
      .push_str(&settings.tone.message);
  }

  if settings.politeness.enabled {
    dev_message
      .content
      .push_str(&settings.politeness.message);
  }

  if settings.formality.enabled {
    dev_message
      .content
      .push_str(&settings.formality.message);
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

#[derive(Deserialize, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
  formality: Criteria,
  grammar: Criteria,
  politeness: Criteria,
  readability: Criteria,
  tone: Criteria,

  token: Option<String>,
}

#[derive(Deserialize, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct Criteria {
  pub message: String,
  pub enabled: bool,
}

#[derive(Default, Serialize)]
struct Prompt {
  model: String,
  messages: Vec<Message>,
}

impl Prompt {
  fn with_messages<I>(messages: I) -> Self
  where
    I: IntoIterator<Item = Message>,
  {
    Self {
      model: "gpt-5-mini".to_owned(),
      messages: messages.into_iter().collect(),
    }
  }
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
