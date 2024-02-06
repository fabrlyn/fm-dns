mod application;
mod cli;
mod model;
mod scanner;
mod stdout;

use cli::run;

#[tokio::main]
async fn main() {
    run().await;
}
