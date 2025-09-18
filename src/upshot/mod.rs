// src/upshot/mod.js
//! module to handling result structrue
//!
//! Current support quiet and normal output mode

use crate::cli::{DisplayConfig, DisplayFormat};
use colored::Colorize;
use phf::phf_map;
use serde::Serialize;
use std::fmt;
use std::io;

static SERVICE_MAP: phf::Map<u16, &'static str> = phf_map! {
    20u16 | 21u16 => "FTP",
    989u16 | 990u16 => "FTPS",
    25u16 => "SMTP",
    465u16 => "SMTPS",
    80u16 => "HTTP",
    443u16 => "HTTPS",
    110u16 => "POP3",
    995u16 => "POP3S",
    119u16 => "NNTP",
    563u16 => "NNTPS",
    143u16 => "IMAP",
    993u16 => "IMAPS",
    22u16 => "SSH",
    23u16 => "TELNET",
    53u16 => "DNS",
    123u16 => "NTP",
};

fn parse_service(port: u16) -> String {
    SERVICE_MAP.get(&port).unwrap_or(&"UNK").to_string()
}

#[derive(Debug, Clone, Serialize)]
pub enum Status {
    OPEN,
    CLOSE,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Status::OPEN => "OPEN".green().bold(),
            Status::CLOSE => "CLOSE".red().bold(),
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Upshot {
    pub target: String,
    pub ip: String,
    pub port: u16,
    pub service: String,
    pub status: Status,
}

impl Upshot {
    pub fn new(target: String, ip: String, port: u16, status: Status) -> Self {
        Upshot {
            target: target,
            ip: ip,
            port: port,
            service: parse_service(port),
            status: status,
        }
    }
}

pub fn upshot_normal(upshot: Upshot) -> String {
    format!(
        "{}\t{}:{}\t{}\t{}\n",
        upshot.target, upshot.ip, upshot.port, upshot.status, upshot.service
    )
}

pub fn upshot_quiet(upshot: Upshot) -> String {
    match upshot.status {
        Status::OPEN => format!("{}\n", upshot.port),
        Status::CLOSE => String::new(),
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
