// src/cli.rs

use crate::scanner;
use clap::Parser;

#[derive(Parser)]
#[command(name = "portscan")]
#[command(about = "A simple port scanner", long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "127.0.0.1")]
    target: String,

    #[arg(short, long, value_delimiter = ',', default_value = "80")]
    ports: Vec<String>,

    #[arg(long, default_value = "500")]
    timeout: u64,
}

fn expand_ports_spec(specs: &Vec<String>) -> Result<Vec<u16>, String> {
    let mut ports: Vec<u16> = Vec::new();
    for spec in specs {
        if spec.contains('-') {
            let mut parts = spec.splitn(2, '-');

            let from = parts.next().unwrap().trim();
            let to = parts.next().unwrap().trim();
            if from.parse::<u16>().is_err() || to.parse::<u16>().is_err() {
                return Err("Invalid port range".to_string());
            }

            let from = from.parse::<u16>().unwrap();
            let to = to.parse::<u16>().unwrap();
            if from > to {
                return Err("Invalid port range".to_string());
            }
            for i in from..=to {
                ports.push(i);
            }
        } else {
            if spec.parse::<u16>().is_err() {
                return Err("Invalid port".to_string());
            }
            let port = spec.parse::<u16>().unwrap();
            ports.push(port);
        }
    }
    Ok(ports)
}

pub fn parse() {
    let args = Args::parse();
    let ports = expand_ports_spec(&args.ports).unwrap();
    for port in ports {
        let target = format!("{}:{}", args.target, port);
        scanner::scan(&target, args.timeout);
    }
}
