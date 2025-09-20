// src/scanner/mod.rs
//! Port scanning engine.
//!
//! Currently supports TCP connect scan.

use crate::cli::AddrConfig;
use crate::upshot::{Status, Upshot};
use colored::Colorize;
use pinger::{PingOptions, PingResult, ping};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

pub fn ping_target(target: &String, timeout: u64) -> bool {
    let options = PingOptions::new(target, Duration::from_millis(timeout), None);
    let stream = ping(options).expect(&format!("{}: {}", "Error".red().bold(), "pinging"));
    for message in stream {
        match message {
            PingResult::Pong(_, _) => return true,
            _ => {}
        }
    }
    false
}

pub fn scan(addr_config: AddrConfig) -> Vec<Upshot> {
    let mut upshots: Vec<Upshot> = Vec::new();

    let addr = format!("{}:{}", addr_config.target, addr_config.port);

    let addrs = match addr.to_socket_addrs() {
        Ok(addrs) => addrs,
        Err(err) => {
            eprintln!(
                "{}: failed to resolve address '{}' - {}",
                "Error".red().bold(),
                addr,
                err.to_string().split(':').next().unwrap(),
            );
            std::process::exit(1);
        }
    };

    for addr in addrs {
        let status: Status;

        match TcpStream::connect_timeout(&addr, Duration::from_millis(addr_config.timeout)) {
            Ok(_) => {
                status = Status::OPEN;
            }
            Err(_) => {
                status = Status::CLOSE;
            }
        }

        upshots.push(Upshot::new(
            addr_config.target.to_string(),
            addr.ip().to_string(),
            addr_config.port,
            status,
        ));
    }

    upshots
}
