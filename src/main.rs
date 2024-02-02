mod args;
mod cli;
mod scanner;
mod stdout;

use async_trait::async_trait;
use cli::Cli;
use futures_util::{pin_mut, StreamExt};

use ractor::{concurrency::JoinHandle, Actor, ActorProcessingErr, ActorRef, OutputPort};
use std::{
    error::Error,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::spawn;

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
