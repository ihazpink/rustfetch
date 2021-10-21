//! Module for getting battery percentage

use colored::*;
use std::fs;

/// Get formatted battery info.
pub fn get_battery() -> String {
    let battery_perc = fs::read_to_string("/sys/class/power_supply/BAT0/capacity").unwrap();
    let battery_perc = battery_perc.trim_end_matches('\n');
    format!("{}: {}%", "Battery".bright_blue(), battery_perc)
}
