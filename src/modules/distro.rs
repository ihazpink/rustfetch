//! Module for getting distro name and logo, as well as system architecture
//!
//! TODO: get system architecture

use crate::helpers::{find_val, unwrap_or_return};
use crate::logo;
use colored::*;

use nix::sys::utsname::UtsName;

use std::fs;

/// Simple distro representation, used to make finding logos easier
#[derive(Debug)]
pub struct Distro {
    pub(self) name: String,
    id:             String,
}

impl Distro {
    /// Finds distro logo based on id
    /// If it doesn't find a matching logo returns a questionmark logo
    pub(self) fn logo(&self) -> &'static str {
        match self.id.as_str() {
            "arch" => logo::ARCH_LOGO,
            "artix" => logo::ARTIX_LOGO,
            _ => "not based",
        }
    }

    /// Returns Distro with both name and id as "Unknown"
    fn unknown() -> Distro {
        Distro {
            name: "Unknown".to_string(),
            id:   "Unknown".to_string(),
        }
    }
}


/// Attempts to find user distro and logo
pub fn get_distro_struct() -> Distro {
    let os_release = unwrap_or_return!(fs::read_to_string("/etc/os-release"), Distro::unknown());

    let name = find_val(&os_release, "PRETTY_NAME", "=")
        .unwrap_or("Unknown".to_string())
        .replace("\"", "");
    let id = find_val(&os_release, "ID", "=").unwrap_or("Unknown".to_string());

    Distro { name, id }
}


pub fn get_distro_and_arch(distro: &Distro, uname: &UtsName) -> String {
    let arch = uname.machine();
    format!("{}: {} {}", "OS".bright_blue(), distro.name, arch)
}

pub fn get_logo(distro: &Distro) -> &'static str { distro.logo() }
