use serde::ser::Serializer;
use serde::Serialize;
use std::error::Error as StdError;
pub use std::result::Result as StdResult;

pub type Result<T> = StdResult<T, Error>;
pub type BoxResult<T> = StdResult<T, Box<dyn StdError>>;
pub type CResult<T> = StdResult<T, String>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("Io: {0}")]
  Io(#[from] std::io::Error),
  #[error("Json: {0}")]
  Json(#[from] serde_json::Error),
  #[error("Reqwest: {0}")]
  Reqwest(#[from] reqwest::Error),
  #[error("Svelte: {0}")]
  Svelte(#[from] tauri_plugin_svelte::Error),
  #[error("Tauri: {0}")]
  Tauri(#[from] tauri::Error),
  #[error("{0}")]
  Unknown(#[from] anyhow::Error),
}

impl Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(self.to_string().as_str())
  }
}

impl From<Error> for String {
  fn from(value: Error) -> Self {
    value.to_string()
  }
}

pub trait WrapErr<E: Into<Error>> {
  fn wrap_err<T>(self) -> Result<T>;
}

impl<E: Into<Error>> WrapErr<E> for E {
  fn wrap_err<T>(self) -> Result<T> {
    Err(self.into())
  }
}

#[macro_export]
macro_rules! err {
  ($($arg:tt)*) => {{
    use $crate::error::WrapErr;
    anyhow::anyhow!($($arg)*).wrap_err()
  }};
}

#[macro_export]
macro_rules! bail {
  ($($arg:tt)*) => {{
    return $crate::err!($($arg)*);
  }};
}
