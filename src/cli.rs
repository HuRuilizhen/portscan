// src/cli.rs

use crate::config::{AddrConfig, DisplayConfig, DisplayFormat};
use crate::{scanner, upshot::Upshot};
use clap::Parser;
use colored::Colorize;

#[derive(Parser)]
#[command(name = "portscan")]
#[command(
    about = "A simple yet powerful TCP port scanner that supports domains and IP addresses",
    version = env!("CARGO_PKG_VERSION"),
    author = "HuRuilizhen",
    help_template = r#"
{about}

Version: {version}
Author: {author}

Usage: {usage}

{all-args}
"#
)]
pub struct Args {
    #[arg(
        short,
        long,
        required = true,
        help = "Target host (IP or domain), e.g. 127.0.0.1 or google.com"
    )]
    target: String,

    #[arg(
        short,
        long,
        value_delimiter = ',',
        default_value = "22",
        help = "Ports to scan. Comma-separated or ranges, e.g. 80,443,8080-8090"
    )]
    ports: Vec<String>,

    #[arg(
        long,
        default_value = "500",
        help = "Connection timeout in milliseconds."
    )]
    timeout: u64,

    #[arg(
        short,
        long,
        default_value = "100",
        help = "Concurrency of port scanning"
    )]
    concurrency: u16,

    #[arg(
        long,
        default_value = "text",
        help = "Output format, one of 'text', 'json' or 'csv'"
    )]
    format: String,

    #[arg(
        long,
        help = "Only open ports will be printed line by line (suitable for scripting)"
    )]
    quiet: bool,

    #[arg(long, help = "Try ping target host before starting port scan")]
    ping: bool,
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

pub fn parse() -> (Vec<Upshot>, DisplayConfig) {
    let args = Args::parse();

    let ports = match expand_ports_spec(&args.ports) {
        Ok(ports) => ports,
        Err(err) => {
            eprintln!("{}: {}", "Error".red().bold(), err);
            std::process::exit(1);
        }
    };

    if args.ping && !scanner::ping_target(&args.target, args.timeout) {
        eprintln!(
            "{}: failed to ping target {}",
            "Error".red().bold(),
            &args.target,
        );
        std::process::exit(0);
    }

    let upshots = scanner::scan_ports(AddrConfig {
        target: args.target.clone(),
        ports: ports,
        timeout: args.timeout,
        concurrency: args.concurrency,
    });

    let mut format = DisplayFormat::Text;
    if args.format == "json" {
        format = DisplayFormat::Json;
    } else if args.format == "csv" {
        format = DisplayFormat::Csv;
    } else if args.format != "text" {
        eprintln!("{}: {}", "Error".red().bold(), "Invalid format");
        std::process::exit(1);
    }

    (
        upshots,
        DisplayConfig {
            format: format,
            quiet: args.quiet,
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_port() {
        assert_eq!(expand_ports_spec(&vec!["80".to_string()]), Ok(vec![80]));
    }

    #[test]
    fn test_range_ports() {
        assert_eq!(
            expand_ports_spec(&vec!["1-100".to_string()]),
            Ok((1..=100).collect())
        );
    }
}
