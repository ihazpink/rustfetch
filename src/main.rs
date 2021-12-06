mod helpers;
mod logo;
mod modules;

use colored::*;
use itertools::EitherOrBoth::*;
use itertools::Itertools;
use nix::sys::{sysinfo, utsname};

use modules::battery::get_battery;
use modules::distro::{get_distro_and_arch, get_distro_struct, get_logo};
use modules::kernel::get_kernel;
use modules::locale::get_locale;
use modules::packages::get_packages;
use modules::seperator::get_seperator;
use modules::title::get_title;
use modules::uptime::get_uptime;
use modules::ifaddrs::get_ifaddrs;
use modules::editor::get_editor;


fn main() {
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
        get_editor(),
    ];
    // Append because get_packages() returns a vector
    config.append(&mut get_packages());
    config.append(&mut get_ifaddrs());

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
