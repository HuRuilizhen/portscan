// src/upshot/mod.js
//! module to handling result structrue
//!
//! Current support quiet and normal output mode

use crate::cli::{DisplayConfig, DisplayFormat};
use colored::Colorize;
use serde::Serialize;
use std::fmt;
use std::io;

#[derive(Debug, Clone, Serialize)]
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

#[derive(Debug, Clone, Serialize)]
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

pub fn display_upshot_json(upshots: Vec<Upshot>) {
    println!("{}", serde_json::to_string(&upshots).unwrap());
}

pub fn display_upshot_csv(upshots: Vec<Upshot>) {
    let mut wtr = csv::Writer::from_writer(io::stdout());

    for upshot in upshots {
        wtr.serialize(upshot).unwrap();
    }

    wtr.flush().unwrap();
}

pub fn display_upshot_text(upshots: Vec<Upshot>, display_config: DisplayConfig) {
    for upshot in upshots {
        match display_config.quiet {
            true => print!("{}", upshot_quiet(upshot)),
            false => {
                print!("{}", upshot_normal(upshot));
            }
        }
    }
}

pub fn display_upshots(upshots: Vec<Upshot>, display_config: DisplayConfig) {
    match display_config.format {
        DisplayFormat::Text => display_upshot_text(upshots, display_config),
        DisplayFormat::Json => display_upshot_json(upshots),
        DisplayFormat::Csv => display_upshot_csv(upshots),
    }
}
