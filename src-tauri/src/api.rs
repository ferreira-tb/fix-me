use crate::command;
use tauri_specta::{collect_commands, Builder as SpectaBuilder, ErrorHandlingMode};

pub fn collect() -> SpectaBuilder {
  let builder = SpectaBuilder::new()
    .error_handling(ErrorHandlingMode::Throw)
    .commands(collect_commands![command::prompt, command::show_window]);

  #[cfg(debug_assertions)]
  export(&builder);

  builder
}

#[cfg(debug_assertions)]
fn export(specta: &SpectaBuilder) {
  use specta_typescript::{BigIntExportBehavior, Typescript};

  let ts = Typescript::default()
    .bigint(BigIntExportBehavior::BigInt)
    .remove_default_header()
    .header("// @ts-nocheck");

  specta
    .export(ts, "../src/lib/api/bindings.ts")
    .expect("failed to export typescript bindings");
}
