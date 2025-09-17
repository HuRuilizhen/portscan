// src/cli.rs

use crate::scanner;
use clap::Parser;
use colored::Colorize;

#[derive(Parser)]
#[command(name = "portscan")]
#[command(about = "A simple port scanner", long_about = None)]
pub struct Args {
    #[arg(short, long, required = true)]
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
            if from == 0 || to == 0 {
                return Err(format!(
                    "Invalid port range {}-{}, zero is not allowed",
                    from, to
                ));
            }
            if from > to {
                return Err(format!("Invalid port range {}-{}, from > to", from, to));
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

    let ports = match expand_ports_spec(&args.ports) {
        Ok(ports) => ports,
        Err(err) => {
            eprintln!("{}: {}", "Error".red().bold(), err);
            std::process::exit(1);
        }
    };

    for port in ports {
        scanner::scan(&args.target, port, args.timeout);
    }
}
