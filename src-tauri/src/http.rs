use anyhow::{Result, bail};
use reqwest::Client;
use reqwest::header::{self, HeaderMap};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::sync::LazyLock;
use tauri::async_runtime::spawn;
use tokio::sync::OwnedSemaphorePermit;
use tokio::time::{Duration, sleep};

const URL: &str = "https://api.openai.com/v1/chat/completions";

const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

pub static HTTP: LazyLock<Client> = LazyLock::new(|| {
  let mut headers = HeaderMap::new();
  headers.insert(header::ACCEPT, "application/json".parse().unwrap());

  Client::builder()
    .use_rustls_tls()
    .https_only(true)
    .default_headers(headers)
    .user_agent(USER_AGENT)
    .timeout(Duration::from_mins(1))
    .build()
    .expect("failed to create http client")
});

pub async fn post<T, B>(permit: OwnedSemaphorePermit, token: &str, body: &B) -> Result<T>
where
  T: DeserializeOwned,
  B: Serialize + ?Sized,
{
  let (status, response) = HTTP
    .post(URL)
    .bearer_auth(token)
    .json(body)
    .send()
    .await
    .map(|response| (response.status(), response))?;

  if !status.is_success() {
    let url = response.url().to_owned();
    let message = response.text().await?;
    bail!("REQUEST FAILED ({status})\nurl: {url}\nreason: {message}");
  }

  spawn(async move {
    sleep(Duration::from_secs(1)).await;
    drop(permit);
  });

  response.json().await.map_err(Into::into)
}
