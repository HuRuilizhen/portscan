mod cli;
mod scanner;
mod upshot;

fn main() {
    let (upshots, display_config): (Vec<upshot::Upshot>, cli::DisplayConfig) = cli::parse();
    upshot::display_upshots(upshots, display_config);
}
