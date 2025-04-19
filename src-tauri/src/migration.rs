use serde_json::json;
use tauri_plugin_pinia::Migration;

use crate::llm::{
  Criteria,
  DEFAULT_FORMALITY,
  DEFAULT_GRAMMAR,
  DEFAULT_POLITENESS,
  DEFAULT_READABILITY,
  DEFAULT_TONE,
};

pub fn v2_1_0() -> Migration {
  Migration::new("2.1.0", |state| {
    let formality = state.try_get_or_default("formality");
    state.set(
      "formality",
      json!(Criteria {
        message: DEFAULT_FORMALITY.to_owned(),
        enabled: formality,
      }),
    );

    let grammar = state.try_get_or_default("grammar");
    state.set(
      "grammar",
      json!(Criteria {
        message: DEFAULT_GRAMMAR.to_owned(),
        enabled: grammar,
      }),
    );

    let politeness = state.try_get_or_default("politeness");
    state.set(
      "politeness",
      json!(Criteria {
        message: DEFAULT_POLITENESS.to_owned(),
        enabled: politeness,
      }),
    );

    let readability = state.try_get_or_default("readability");
    state.set(
      "readability",
      json!(Criteria {
        message: DEFAULT_READABILITY.to_owned(),
        enabled: readability,
      }),
    );

    let tone = state.try_get_or_default("tone");
    state.set(
      "tone",
      json!(Criteria {
        message: DEFAULT_TONE.to_owned(),
        enabled: tone,
      }),
    );

    Ok(())
  })
}
