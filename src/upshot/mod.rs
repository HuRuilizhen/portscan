// src/upshot/mod.js
//! module to handling result structrue
//!
//! Current support quiet and normal output mode

use crate::cli::DisplayConfig;
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
        "{}\t{}:{}\t{}\n",
        upshot.target, upshot.ip, upshot.port, upshot.status,
    )
}

pub fn upshot_quiet(upshot: Upshot) -> String {
    match upshot.status {
        Status::OPEN => return format!("{}\n", upshot.port),
        Status::CLOSE => return "".to_string(),
    }
}

pub fn display_upshots(upshots: Vec<Upshot>, display_config: DisplayConfig) {
    for upshot in upshots {
        match display_config.quiet {
            true => print!("{}", upshot_quiet(upshot)),
            false => {
                print!("{}", upshot_normal(upshot));
            }
        }
    }
}
