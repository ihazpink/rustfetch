//! Module for getting info in $EDITOR

use std::env;
use colored::*;

/// Returns formatted value of $EDITOR
pub fn get_editor() -> String {
    let editor = env::var("EDITOR")
        .unwrap_or_else(|_| String::new());

    format!("{}: {}", "$EDITOR".bright_blue(), editor)
}
