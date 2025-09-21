#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
#![feature(try_blocks)]

mod api;
mod command;
mod error;
mod http;
mod llm;
mod plugin;
mod tray;
mod window;

use error::BoxResult;
use llm::Llm;
use tauri::{AppHandle, Manager};

fn main() {
  let specta = api::collect();
  tauri::Builder::default()
    .plugin(plugin::single_instance())
    .plugin(plugin::pinia())
    .plugin(plugin::prevent_default())
    .plugin(plugin::window_state())
    .plugin(tauri_plugin_clipboard_manager::init())
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_process::init())
    .plugin(tauri_plugin_updater::Builder::new().build())
    .setup(|app| setup(app.app_handle()))
    .invoke_handler(specta.invoke_handler())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn setup(app: &AppHandle) -> BoxResult<()> {
  app.manage(Llm::new(app.clone()));
  window::open(app)?;
  Ok(())
}
