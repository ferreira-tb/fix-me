use tauri::{Manager, WebviewWindow, Wry};

pub trait ManagerExt: Manager<Wry> {
  fn main_window(&self) -> WebviewWindow<Wry> {
    self.get_webview_window("main").unwrap()
  }
}

impl<T: Manager<Wry>> ManagerExt for T {}
