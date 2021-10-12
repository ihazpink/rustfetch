//! Module for getting the title (username@hostname)

use colored::*;
use nix::sys::utsname::UtsName;
use nix::unistd::{geteuid, User};

pub(crate) fn get_username() -> String {
    let uid = geteuid();
    let user = User::from_uid(uid).unwrap().unwrap();
    user.name
}

pub(crate) fn get_hostname(uname: &UtsName) -> String { uname.nodename().to_string() }

/// Returns formatted title
pub fn get_title(uname: &UtsName) -> String {
    format!(
        "{}@{}",
        get_username().bright_blue(),
        get_hostname(uname).bright_blue()
    )
}
