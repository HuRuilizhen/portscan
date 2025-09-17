// src/upshot/mod.js
//! module to handling result structrue
//!
//! Current support quiet and normal output mode

use colored::Colorize;
use std::fmt;

pub enum Status {
    OPEN,
    CLOSE,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Status::OPEN => "OPEN".green().bold().to_string(),
            Status::CLOSE => "CLOSE".red().bold().to_string(),
        };
        write!(f, "{}", s)
    }
}

pub struct Upshot {
    pub target: String,
    pub ip: String,
    pub port: u16,
    pub status: Status,
}

pub fn upshot_normal(upshot: Upshot) -> String {
    format!(
        "{}\t{}:{}\t{}",
        upshot.target, upshot.ip, upshot.port, upshot.status,
    )
}

pub fn display_upshots(upshots: Vec<Upshot>) {
    for upshot in upshots {
        println!("{}", upshot_normal(upshot));
    }
}
