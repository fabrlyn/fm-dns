use crate::{
    model::ServiceQuery,
    scanner::{self, Port, Scanner},
    stdout::Stdout,
};
use ractor::{concurrency::JoinHandle, Actor};
use std::{sync::Arc, time::Duration};

#[derive(Debug)]
pub struct Config {
    pub interval: Duration,
    pub service_query: Arc<ServiceQuery>,
}

pub async fn run(config: Config) {
    let port = Port::default();

    start_stdout(port.clone()).await;

    let handle = start_scanner(&config, port).await;

    handle
        .await
        .expect("Ping-pong actor failed to exit properly");
}

async fn start_scanner(config: &Config, port: Port) -> JoinHandle<()> {
    let (_, handle) = Actor::spawn(
        None,
        Scanner,
        scanner::Arguments {
            port: port.clone(),
            service_query: config.service_query.clone(),
            interval: config.interval,
        },
    )
    .await
    .expect("Failed to start ping-pong actor");

    handle
}

async fn start_stdout(port: Port) -> JoinHandle<()> {
    let (stdout, handle) = Actor::spawn(None, Stdout, ())
        .await
        .expect("Failed to start ping-pong actor");

    port.subscribe(stdout, Some);
    handle
}
