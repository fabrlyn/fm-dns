mod args;
mod cli;
mod scanner;
mod stdout;


use cli::Cli;



use std::{
    error::Error,
};


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Cli::run().await;

    //let output = Arc::new(OutputPort::default());

    //let (_actor, handle) = Actor::spawn(None, Scanner, output.clone())
    //    .await
    //    .expect("Failed to start ping-pong actor");

    //let (_actor, handle) = Actor::spawn(None, StdoutPublisher, output)
    //    .await
    //    .expect("Failed to start ping-pong actor");
    //handle
    //    .await
    //    .expect("Ping-pong actor failed to exit properly");

    Ok(())
}
