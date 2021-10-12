//! Module for finding package manager and number of packages
//!
//! Currently supports: pacman, dpkg, xbps, snap, flatpak
use std::path::Path;
use std::{fmt, fs};

use crate::helpers::unwrap_or_return;
use colored::*;

/// Type of entry in directory
enum EntryType {
    File,
    Dir,
}

/// Gets either number of files or directories in a given directory
fn get_num_elements<P: AsRef<Path>>(path: P, entry_type: EntryType) -> usize {
    let mut counter = 0;
    for entry in unwrap_or_return!(fs::read_dir(path), 0) {
        let entry = unwrap_or_return!(entry, 0);
        let path = entry.path();
        match entry_type {
            EntryType::File => {
                if path.is_file() {
                    counter += 1
                }
            }
            EntryType::Dir => {
                if path.is_dir() {
                    counter += 1
                }
            }
        }
    }
    counter
}

fn get_num_strings<P: AsRef<Path>>(path: P, needle: &'static str) -> usize {
    let file = unwrap_or_return!(fs::read_to_string(path), 0);
    file.matches(needle).count()
}

#[derive(Debug)]
struct PackageManager {
    pub name:     &'static str,
    pub packages: usize,
}

impl PackageManager {
    pub fn new(name: &'static str, packages: usize) -> PackageManager {
        PackageManager { name, packages }
    }
}

impl fmt::Display for PackageManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.packages, self.name)
    }
}

/// Attempts to find number of packages and package manager \
///
/// Currently supports: pacman, dpkg, xbps, flatpak, snap
pub fn get_packages() -> Vec<String> {
    let mut package_managers = vec![ 
    PackageManager::new(
        "pacman",
        get_num_elements("/var/lib/pacman/local", EntryType::Dir)),
    PackageManager::new(
        "dpkg",
        get_num_strings("/var/lib/dpkg/status", "Status: ")),
    PackageManager::new(
        "xbps",
        get_num_elements("/var/db/xbps", EntryType::File)),
    PackageManager::new(
        "flatpak",
        get_num_elements("/var/lib/flatpak/app", EntryType::Dir)),
    PackageManager::new(
        "snap",
        get_num_elements("/snap", EntryType::Dir)),
    ];

    package_managers = package_managers
        .into_iter()
        .filter(|p| p.packages != 0)
        .collect();

    if package_managers.is_empty() {
        panic!("Package manager not found!")
    }

    let mut result = Vec::new();
    for pm in package_managers {
        result.push(format!("{}: {}", "Packages".bright_blue(), pm))
    }
    result
}
