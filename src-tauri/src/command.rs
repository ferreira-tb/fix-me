use crate::error::{CResult, Error};
use crate::llm::Llm;
use tauri::{AppHandle, Manager, WebviewWindow};

#[tauri::command]
#[specta::specta]
pub async fn prompt(app: AppHandle, prompt: String) -> CResult<String> {
  app
    .state::<Llm>()
    .prompt(prompt)
    .await
    .map_err(Into::into)
}

#[tauri::command]
#[specta::specta]
pub async fn show_window(window: WebviewWindow) -> CResult<()> {
  window
    .show()
    .map_err(Into::<Error>::into)
    .map_err(Into::into)
}
