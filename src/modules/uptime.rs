//! Module for getting and formatting uptime

use colored::*;
use humantime::format_duration;
use nix::sys::sysinfo::SysInfo;

pub fn get_uptime(sysinfo: &SysInfo) -> String {
    let uptime = format_duration(sysinfo.uptime()).to_string();
    format!("{}: {}", "Uptime".bright_blue(), uptime)
}
