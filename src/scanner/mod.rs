// src/scanner/mod.rs
//! Port scanning engine.
//!
//! Currently supports TCP connect scan.

use crate::cli::AddrConfig;
use crate::upshot::{Status, Upshot};
use colored::Colorize;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

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

        upshots.push(Upshot {
            target: addr_config.target.to_string(),
            ip: addr.ip().to_string(),
            port: addr_config.port,
            status: status,
        });
    }

    upshots
}
