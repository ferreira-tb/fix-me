#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
#![feature(duration_constructors, let_chains, try_blocks)]

mod api;
mod command;
mod error;
mod http;
mod llm;

use error::{BoxResult, Result};
use llm::Llm;
use std::time::Duration;
use tauri::plugin::TauriPlugin;
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder, Wry};

fn main() {
  let specta = api::collect();
  tauri::Builder::default()
    .plugin(svelte())
    .plugin(prevent_default())
    .plugin(window_state())
    .plugin(tauri_plugin_process::init())
    .setup(|app| setup(app.app_handle()))
    .invoke_handler(specta.invoke_handler())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn setup(app: &AppHandle) -> BoxResult<()> {
  app.manage(Llm::new(app.clone()));
  open_window(app)?;
  Ok(())
}

fn open_window(app: &AppHandle) -> Result<()> {
  let url = WebviewUrl::App("index.html".into());
  WebviewWindowBuilder::new(app, "main", url)
    .title("Fix Me")
    .inner_size(800.0, 600.0)
    .resizable(false)
    .maximizable(false)
    .minimizable(true)
    .visible(false)
    .build()?;

  Ok(())
}

fn svelte() -> TauriPlugin<Wry> {
  use tauri_plugin_svelte::SaveStrategy;

  tauri_plugin_svelte::Builder::new()
    .autosave(Duration::from_mins(5))
    .default_save_strategy(SaveStrategy::debounce_secs(1))
    .pretty(true)
    .build()
}

#[cfg(windows)]
fn prevent_default() -> TauriPlugin<Wry> {
  tauri_plugin_prevent_default::Builder::new()
    .general_autofill(false)
    .password_autosave(false)
    .build()
}

#[cfg(not(windows))]
fn prevent_default() -> TauriPlugin<Wry> {
  tauri_plugin_prevent_default::init()
}

fn window_state() -> TauriPlugin<Wry> {
  use tauri_plugin_window_state::StateFlags as Flags;

  tauri_plugin_window_state::Builder::new()
    .with_state_flags(Flags::POSITION)
    .build()
}