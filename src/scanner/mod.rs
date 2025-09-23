// src/scanner/mod.rs
//! Port scanning engine.
//!
//! Currently supports TCP connect scan.

use crate::config::{AddrConfig, ScanConfig};
use crate::upshot::{Status, Upshot};
use colored::Colorize;
use pinger::{PingOptions, PingResult, ping};
use std::net::{TcpStream, ToSocketAddrs};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;

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

type ConcurrencyLimit = Arc<Semaphore>;

pub async fn scan_port(scan_config: ScanConfig, limit: ConcurrencyLimit) -> Vec<Upshot> {
    let permit = limit.acquire().await.unwrap();

    let mut upshots: Vec<Upshot> = Vec::new();

    let addr = format!("{}:{}", scan_config.target, scan_config.port);

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

        match TcpStream::connect_timeout(&addr, Duration::from_millis(scan_config.timeout)) {
            Ok(_) => {
                status = Status::OPEN;
            }
            Err(_) => {
                status = Status::CLOSE;
            }
        }

        upshots.push(Upshot::new(
            scan_config.target.to_string(),
            addr.ip().to_string(),
            scan_config.port,
            status,
        ));
    }

    drop(permit);

    upshots
}

pub async fn scan_ports(addr_config: AddrConfig) -> Vec<Upshot> {
    let limit = Arc::new(Semaphore::new(addr_config.concurrency.into()));

    let mut handles = Vec::new();

    for port in addr_config.ports {
        let limit = limit.clone();
        let target = addr_config.target.clone();

        let handle = tokio::spawn(async move {
            scan_port(
                ScanConfig {
                    target: target,
                    port: port,
                    timeout: addr_config.timeout,
                },
                limit,
            )
            .await
        });
        handles.push(handle);
    }

    let mut upshots: Vec<Upshot> = Vec::new();

    for handle in handles {
        upshots.append(&mut handle.await.unwrap());
    }

    upshots
}
