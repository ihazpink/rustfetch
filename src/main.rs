mod helpers;
mod logo;
mod modules;

use colored::*;
use itertools::EitherOrBoth::*;
use itertools::Itertools;
use modules::distro::{get_distro_and_arch, get_distro_struct, get_logo};
use modules::kernel::get_kernel;
use modules::packages::get_packages;
use modules::seperator::get_seperator;
use modules::title::get_title;
use modules::uptime::get_uptime;
use nix::sys::{sysinfo, utsname};

fn main() {
    let mut config: Vec<String> = Vec::new();

    let distro = get_distro_struct();
    let uname = utsname::uname();
    let sysinfo = sysinfo::sysinfo().unwrap();

    config.push(get_title(&uname));
    config.push(get_seperator(&uname));
    config.push(get_distro_and_arch(&distro, &uname));
    config.push(get_kernel(&uname));
    config.push(get_uptime(&sysinfo));

    // Append because get_packages() returns a vector
    config.append(&mut get_packages());
    let logo = get_logo(&distro);

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
