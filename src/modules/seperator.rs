use nix::sys::utsname::UtsName;

use super::title::{get_hostname, get_username};

pub fn get_seperator(uname: &UtsName) -> String {
    // +1 to account for @ sign
    "-".repeat(get_username().len() + get_hostname(uname).len() + 1)
}
