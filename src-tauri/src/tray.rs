use crate::manager::ManagerExt;
use anyhow::Result;
use tauri::menu::{Menu, MenuBuilder, PredefinedMenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Manager, Wry, include_image};

const ID: &str = "fix-me-tray-icon";

struct TrayMenu(Menu<Wry>);

impl TrayMenu {
  pub fn new<M: ManagerExt>(app: &M) -> Result<Self> {
    MenuBuilder::new(app)
      .items(&[&PredefinedMenuItem::quit(app, None)?])
      .build()
      .map(Self)
      .map_err(Into::into)
  }
}

pub fn create(app: &AppHandle) -> Result<()> {
  if app.tray_by_id(ID).is_some() {
    return Ok(());
  }

  TrayIconBuilder::with_id(ID)
    .tooltip(env!("CARGO_PKG_NAME"))
    .icon(include_image!("./icons/32x32.png"))
    .on_tray_icon_event(on_tray_event())
    .build(app)?;

  Ok(())
}

fn on_tray_event() -> impl Fn(&TrayIcon, TrayIconEvent) {
  move |icon, event| {
    let app = icon.app_handle();
    if let TrayIconEvent::Click { button, button_state, .. } = event {
      if button == MouseButton::Left && button_state == MouseButtonState::Down {
        on_left_click(app);
      } else if button == MouseButton::Right && button_state == MouseButtonState::Down {
        on_right_click(app);
      }
    }
  }
}

fn on_left_click(app: &AppHandle) {
  let _: Result<()> = try {
    let window = app.main_window();
    if !window.is_visible()? {
      window.show()?;
    }

    if window.is_minimized()? {
      window.unminimize()?;
    }

    window.set_focus()?;
  };
}

fn on_right_click(app: &AppHandle) {
  // https://github.com/tauri-apps/tauri/issues/9470#issuecomment-2111339835
  let window = app.main_window();
  let _: Result<()> = try {
    if let Some(menu) = app.try_state::<TrayMenu>() {
      window.popup_menu(&menu.0)?;
    } else if let Ok(menu) = TrayMenu::new(app) {
      window.popup_menu(&menu.0)?;
      app.manage(menu);
    }
  };
}
