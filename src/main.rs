mod args;
mod cli;
pub mod scanner;
mod stdout;

use cli::Cli;

#[tokio::main]
async fn main() {
    Cli::run().await;
}
