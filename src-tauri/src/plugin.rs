use crate::window::WindowExt;
use std::time::Duration;
use tauri::Wry;
use tauri::plugin::TauriPlugin;
use tauri_plugin_pinia::PrettyTomlMarshaler;

pub fn pinia() -> TauriPlugin<Wry> {
  use tauri_plugin_pinia::SaveStrategy;

  tauri_plugin_pinia::Builder::new()
    .autosave(Duration::from_mins(5))
    .default_save_strategy(SaveStrategy::debounce_secs(1))
    .marshaler(Box::new(PrettyTomlMarshaler))
    .build()
}

pub fn prevent_default() -> TauriPlugin<Wry> {
  use tauri_plugin_prevent_default::{Builder, Flags, PlatformOptions};

  Builder::new()
    .with_flags(Flags::debug())
    .platform(
      PlatformOptions::new()
        .browser_accelerator_keys(false)
        .default_context_menus(false)
        .default_script_dialogs(false)
        .general_autofill(false)
        .password_autosave(false)
        .zoom_control(false),
    )
    .build()
}

pub fn single_instance() -> TauriPlugin<Wry> {
  tauri_plugin_single_instance::init(|app, _, _| {
    app.main_window().show().unwrap();
  })
}
