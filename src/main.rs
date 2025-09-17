mod cli;
mod scanner;
mod upshot;

fn main() {
    upshot::display_upshots(cli::parse());
}
