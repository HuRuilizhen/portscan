// src/cli.rs

use crate::scanner;
use clap::Parser;

#[derive(Parser)]
#[command(name = "portscan")]
#[command(about = "A simple port scanner", long_about = None)]
pub struct Args {
    #[arg(short, long, default_value_t = String::from("127.0.0.1"))]
    target: String,

    #[arg(short, long, default_value_t = 80)]
    port: u16,
}

pub fn parse() {
    let args = Args::parse();
    let target = format!("{}:{}", args.target, args.port);
    scanner::scan(&target);
}
