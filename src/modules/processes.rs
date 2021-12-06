//! Module for getting number of running processes

use colored::*;
use nix::sys::sysinfo::SysInfo;

/// Returns number of running processes as a formatted string
pub fn get_processes(sysinfo: &SysInfo) -> String {
    format!("{}: {}", "Processes".bright_blue(), sysinfo.process_count())
}
