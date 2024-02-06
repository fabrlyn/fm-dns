use std::{sync::Arc, time::Duration};

use clap::{command, Parser};
use ractor::{concurrency::JoinHandle, Actor};

use crate::{
    args::ServiceQuery,
    scanner::{self, Port, Scanner},
    stdout::Stdout,
};

#[derive(Debug, Parser)]
#[command(
    about = "Query mdns services on the network",
    name = "fm-dns",
    author = "fabrlyn"
)]
pub struct Cli {
    #[arg(
        help = "The service to query for on the network. Example: _googlecast._tcp.local",
        value_parser = parse_service_query 
    )]
    service_query: Arc<ServiceQuery>,
}

impl Cli {
    pub async fn run() {
        let cli = Cli::parse();

        let port = Port::default();

        start_stdout(port.clone()).await;

        let handle = start_scanner(&cli, port).await;

        handle
            .await
            .expect("Ping-pong actor failed to exit properly");
    }
}

async fn start_scanner(cli: &Cli, port: Port) -> JoinHandle<()> {
    let (_, handle) = Actor::spawn(
        None,
        Scanner,
        scanner::Arguments {
            port: port.clone(),
            service_query: cli.service_query.clone(),
            interval: Duration::from_secs(5),
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

fn parse_service_query(input: &str) -> Result<Arc<ServiceQuery>, String> {
    ServiceQuery::decode(input)
        .map(Arc::new)
        .ok_or("Invalid service query, needs to in the format: _service._proto.domain".to_string())
}
