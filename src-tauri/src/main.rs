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
mod manager;
mod migration;

#[cfg(not(target_os = "linux"))]
mod tray;

use anyhow::Result;
use error::BoxResult;
use llm::Llm;
use manager::ManagerExt;
use std::time::Duration;
use tauri::plugin::TauriPlugin;
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder, Wry};

fn main() {
  let specta = api::collect();
  tauri::Builder::default()
    .plugin(single_instance())
    .plugin(pinia())
    .plugin(prevent_default())
    .plugin(window_state())
    .plugin(tauri_plugin_clipboard_manager::init())
    .plugin(tauri_plugin_dialog::init())
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

  #[cfg_attr(target_os = "linux", allow(unused_variables))]
  let window = WebviewWindowBuilder::new(app, "main", url)
    .title(format!("Fix Me {}", env!("CARGO_PKG_VERSION")))
    .inner_size(800.0, 600.0)
    .resizable(false)
    .maximizable(false)
    .minimizable(true)
    .visible(false)
    .build()?;

  #[cfg(not(target_os = "linux"))]
  window.on_window_event(on_window_event(app));

  Ok(())
}

fn pinia() -> TauriPlugin<Wry> {
  use tauri_plugin_pinia::SaveStrategy;

  tauri_plugin_pinia::Builder::new()
    .autosave(Duration::from_mins(5))
    .default_save_strategy(SaveStrategy::debounce_secs(1))
    .migration("settings", migration::v2_1_0())
    .pretty(true)
    .build()
}

#[cfg(windows)]
fn prevent_default() -> TauriPlugin<Wry> {
  #[cfg(windows)]
  use tauri_plugin_prevent_default::PlatformOptions;
  use tauri_plugin_prevent_default::{Builder, Flags};

  let builder = Builder::new().with_flags(Flags::debug());

  #[cfg(windows)]
  let builder = builder.platform(PlatformOptions {
    general_autofill: false,
    password_autosave: false,
  });

  builder.build()
}

fn single_instance() -> TauriPlugin<Wry> {
  tauri_plugin_single_instance::init(|app, _, _| {
    app.main_window().show().unwrap();
  })
}

fn window_state() -> TauriPlugin<Wry> {
  use tauri_plugin_window_state::StateFlags as Flags;

  tauri_plugin_window_state::Builder::new()
    .with_state_flags(Flags::POSITION)
    .build()
}

#[cfg(not(target_os = "linux"))]
fn on_window_event(app: &AppHandle) -> impl Fn(&tauri::WindowEvent) + use<> {
  use tauri::WindowEvent::CloseRequested;
  let app = app.clone();
  move |event| {
    if !cfg!(debug_assertions)
      && let CloseRequested { api, .. } = event
    {
      api.prevent_close();
      app.main_window().hide().unwrap();
    }
  }
}
