use std::time::Duration;

use clap::{command, Parser};
use ractor::Actor;

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
    service_query: ServiceQuery,
}

impl Cli {
    pub async fn run() {
        let cli = Cli::parse();

        let port = Port::default();

        let (_actor, _) = Actor::spawn(
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

        let (_actor, handle) = Actor::spawn(None, Stdout, port)
            .await
            .expect("Failed to start ping-pong actor");
        handle
            .await
            .expect("Ping-pong actor failed to exit properly");
    }
}

fn parse_service_query(input: &str) -> Result<ServiceQuery, String> {
    ServiceQuery::decode(input)
        .ok_or("Invalid service query, needs to in the format: _service._proto.domain".to_string())
}
