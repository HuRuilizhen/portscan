// src/scanner/mod.rs
//! Port scanning engine.
//!
//! Currently supports TCP connect scan.

use colored::Colorize;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

pub fn scan(target: &str, port: u16, timeout: u64) {
    let addr = format!("{}:{}", target, port);

    let addrs = match addr.to_socket_addrs() {
        Ok(addrs) => addrs,
        Err(err) => {
            eprintln!(
                "{}: failed to resolve address '{}': {}",
                "Error".red().bold(),
                addr,
                err
            );
            return;
        }
    };

    for addr in addrs {
        match TcpStream::connect_timeout(&addr, Duration::from_millis(timeout)) {
            Ok(_) => {
                println!(
                    "{}:{}\t{}\t(via {})",
                    &target,
                    port,
                    "open".green().bold(),
                    addr.ip()
                );
                return;
            }
            Err(_) => {
                println!(
                    "{}:{}\t{}\t(via {})",
                    &target,
                    port,
                    "closed".red().dimmed(),
                    addr.ip()
                );
                continue;
            }
        }
    }
}
