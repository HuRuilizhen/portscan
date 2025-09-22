mod cli;
mod config;
mod scanner;
mod upshot;

#[tokio::main]
async fn main() {
    let (upshots, display_config): (Vec<upshot::Upshot>, config::DisplayConfig) =
        cli::parse().await;
    upshot::display_upshots(upshots, display_config);
}
