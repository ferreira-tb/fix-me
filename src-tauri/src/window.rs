use anyhow::Result;
use serde_json::json;
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindow, WebviewWindowBuilder, Wry};

pub trait WindowExt: Manager<Wry> {
  fn main_window(&self) -> WebviewWindow<Wry> {
    self.get_webview_window("main").unwrap()
  }
}

impl<T: Manager<Wry>> WindowExt for T {}

pub fn open(app: &AppHandle) -> Result<()> {
  let url = WebviewUrl::App("index.html".into());

  let window = WebviewWindowBuilder::new(app, "main", url)
    .title("Fix Me")
    .initialization_script(script())
    .inner_size(800.0, 600.0)
    .resizable(false)
    .maximizable(false)
    .minimizable(true)
    .visible(false)
    .build()?;

  window.on_window_event(on_window_event(app));

  Ok(())
}

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

fn script() -> String {
  let mut script = String::new();
  macro_rules! define {
    ($name:literal, $value:expr) => {{
      let name = $name;
      let value = json!($value);
      let snippet = format! {"
        Object.defineProperty(window, '{name}', {{
          configurable: false,
          enumerable: true,
          writable: false,
          value: {value},
        }});
      "};

      script.push_str(&snippet);
    }};
  }

  define!("__DEBUG_ASSERTIONS__", cfg!(debug_assertions));

  script
}
