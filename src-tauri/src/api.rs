use crate::command;
use tauri_specta::{Builder as SpectaBuilder, ErrorHandlingMode, collect_commands};

use crate::llm::{
  DEFAULT_FORMALITY,
  DEFAULT_GRAMMAR,
  DEFAULT_POLITENESS,
  DEFAULT_READABILITY,
  DEFAULT_TONE,
  Settings,
};

pub fn collect() -> SpectaBuilder {
  let builder = SpectaBuilder::new()
    .error_handling(ErrorHandlingMode::Throw)
    .constant("DEFAULT_FORMALITY", DEFAULT_FORMALITY)
    .constant("DEFAULT_GRAMMAR", DEFAULT_GRAMMAR)
    .constant("DEFAULT_POLITENESS", DEFAULT_POLITENESS)
    .constant("DEFAULT_READABILITY", DEFAULT_READABILITY)
    .constant("DEFAULT_TONE", DEFAULT_TONE)
    .typ::<Settings>()
    .commands(collect_commands![
      command::create_tray_icon,
      command::prompt,
      command::show_window
    ]);

  #[cfg(debug_assertions)]
  export(&builder);

  builder
}

#[cfg(debug_assertions)]
fn export(specta: &SpectaBuilder) {
  use specta_typescript::{BigIntExportBehavior, Typescript};

  let ts = Typescript::default()
    .bigint(BigIntExportBehavior::BigInt)
    .header("// @ts-nocheck");

  specta
    .export(ts, "../src/lib/api/bindings.ts")
    .expect("failed to export typescript bindings");
}
