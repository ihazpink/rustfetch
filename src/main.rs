mod helpers;
mod logo;
mod modules;

use colored::*;
use itertools::EitherOrBoth::*;
use itertools::Itertools;
use nix::sys::{sysinfo, utsname};

use serde::Deserialize;

use modules::battery::get_battery;
use modules::distro::{get_distro_and_arch, get_distro_struct, get_logo};
use modules::kernel::get_kernel;
use modules::locale::get_locale;
use modules::packages::get_packages;
use modules::seperator::get_seperator;
use modules::title::get_title;
use modules::uptime::get_uptime;


#[derive(Debug, Deserialize, Default)]
pub struct Config {
    pub modules:    Option<Vec<String>>,
    pub print_logo: Option<bool>,
}


fn main() {
    let config_path = dirs::config_dir().unwrap().join("rustfetch/rustfetch.toml");
    let config = std::fs::read_to_string(config_path).unwrap_or_default();
    let config: Config = toml::from_str(&config).unwrap_or_default();

    println!("{:#?}", config);

    let distro = get_distro_struct();
    let uname = utsname::uname();
    let sysinfo = sysinfo::sysinfo().unwrap();


    let logo = get_logo(&distro);
    let mut config = vec![
        get_title(&uname),
        get_seperator(&uname),
        get_distro_and_arch(&distro, &uname),
        get_kernel(&uname),
        get_uptime(&sysinfo),
        get_battery(),
        get_locale(),
    ];
    // Append because get_packages() returns a vector
    config.append(&mut get_packages());

    print_logo_and_config(config, logo);
}

fn print_logo_and_config(config: Vec<String>, logo: &'static str) {
    for pair in logo.lines().zip_longest(config.iter()) {
        match pair {
            Both(l, r) => println!("{0: <40}{1: <40}", l.bright_blue(), r),
            Left(l) => println!("{0: <40}{1: <40}", l.bright_blue(), ""),
            Right(r) => println!("{0: <40}{1: <40}", "", r),
        }
    }
}
