//! Module for getting info about system locale

/// Get locale from environment variables, used as a fallback
fn get_locale_from_env() -> String {
    use std::collections::HashMap;
    use std::env;

    let env_vars = env::vars().collect::<HashMap<String, String>>();
    let getenv = |varname| match env_vars.get(varname) {
        Some(val) => val.to_string(),
        None => String::new(),
    };

    let locale = getenv("LANG");
    if !locale.is_empty() {
        return locale;
    }

    let locale = getenv("LC_ALL");
    if !locale.is_empty() {
        return locale;
    }

    let locale = getenv("LC_CTYPE");
    if !locale.is_empty() {
        return locale;
    }

    let locale = getenv("LC_MESSAGES");
    if !locale.is_empty() {
        return locale;
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
