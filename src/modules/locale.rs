//! Module for getting info about system locale

/// Get locale from environment variables, used as a fallback
fn get_locale_from_env() -> String {
    use std::env;

    for key in &["LANG", "LC_ALL", "LC_CTYPE", "LC_MESSAGES"] {
        if let Ok(locale) = env::var(key) {
            return locale;
        }
    }

    panic!("Couldn't find locale!");
}


/// Get formatted info about system locale
pub fn get_locale() -> String {
    use crate::helpers::find_val;
    use colored::*;
    use std::fs;

    let locale_conf = fs::read_to_string("/etc/locale.conf").unwrap_or_default();
    let locale = find_val(&locale_conf, "LANG", "=").unwrap_or_else(|_| get_locale_from_env());

    format!("{}: {}", "Locale".bright_blue(), locale)
}
