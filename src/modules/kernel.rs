//! Module for getting kernel version

use colored::*;
use nix::sys::utsname::UtsName;

pub fn get_kernel(uname: &UtsName) -> String {
    format!("{}: {}", "Kernel".bright_blue(), uname.release())
}
