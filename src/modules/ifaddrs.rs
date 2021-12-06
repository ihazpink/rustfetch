//! Module for getting IP addresses
//!
//! Currently unfinished.
//! TODO: Make printing of IPv6 addresses configurable
//! TODO: Prettier printing of addresses (don't include :0)

use nix::ifaddrs;
use nix::sys::socket::SockAddr;
use colored::*;

fn is_inet(addr: SockAddr) -> bool {
    match addr {
        SockAddr::Inet(_) => true,
        _ => false
    }
}

/// Attempts to get ip addresses 
pub fn get_ifaddrs() -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    let iaddrs = ifaddrs::getifaddrs()
        .expect("ERROR: No IP addresses found.");

    for ifaddr in iaddrs {
        // Skip loop back address
        if ifaddr.interface_name == "lo" { continue }

        // Skip interface if address has a unsupported address family
        let sock_addr = match ifaddr.address {
            Some(address) => address,
            None => continue,
        };

        if is_inet(sock_addr) {
            output.push(
                format!("{}: {}",
                        ifaddr.interface_name.as_str().bright_blue(), sock_addr)
            );
        }
    }

    output
}
