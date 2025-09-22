mod cli;
mod config;
mod scanner;
mod upshot;

fn main() {
    let (upshots, display_config): (Vec<upshot::Upshot>, config::DisplayConfig) = cli::parse();
    upshot::display_upshots(upshots, display_config);
}
