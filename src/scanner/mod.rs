// src/scanner/mod.rs
//! Port scanning engine.
//!
//! Currently supports TCP connect scan.

use std::net::TcpStream;
use std::str::FromStr;
use std::time::Duration;

pub fn scan(addr: &str) {
    let target_addr = std::net::SocketAddr::from_str(addr).unwrap();
    match TcpStream::connect_timeout(&target_addr, Duration::from_millis(500)) {
        Ok(_) => println!("{} open", addr),
        Err(_) => println!("{} closed", addr),
    }
}
