use crate::window::WindowExt;
use std::time::Duration;
use tauri::Wry;
use tauri::plugin::TauriPlugin;

pub fn pinia() -> TauriPlugin<Wry> {
  use tauri_plugin_pinia::SaveStrategy;

  tauri_plugin_pinia::Builder::new()
    .autosave(Duration::from_mins(5))
    .default_save_strategy(SaveStrategy::debounce_secs(1))
    .pretty(true)
    .build()
}

pub fn prevent_default() -> TauriPlugin<Wry> {
  use tauri_plugin_prevent_default::{Builder, Flags, PlatformOptions};

  Builder::new()
    .with_flags(Flags::debug())
    .platform(PlatformOptions {
      general_autofill: false,
      password_autosave: false,
    })
    .build()
}

pub fn single_instance() -> TauriPlugin<Wry> {
  tauri_plugin_single_instance::init(|app, _, _| {
    app.main_window().show().unwrap();
  })
}

pub fn window_state() -> TauriPlugin<Wry> {
  use tauri_plugin_window_state::StateFlags as Flags;

  tauri_plugin_window_state::Builder::new()
    .with_state_flags(Flags::POSITION)
    .build()
}
