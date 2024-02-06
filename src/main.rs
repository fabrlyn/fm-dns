mod args;
mod cli;
mod scanner;
mod stdout;

use cli::Cli;

#[tokio::main]
async fn main() {
    Cli::run().await;
}
