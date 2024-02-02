use std::sync::Arc;

use clap::{command, Parser};
use ractor::{Actor, OutputPort};

use crate::{
    args::ServiceQuery,
    scanner::{self, Scanner},
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

        let output = Arc::new(OutputPort::default());

        let (_actor, handle) = Actor::spawn(
            None,
            Scanner,
            scanner::Arguments {
                output_port: output.clone(),
                service_query: cli.service_query.clone(),
            },
        )
        .await
        .expect("Failed to start ping-pong actor");

        let (_actor, handle) = Actor::spawn(None, Stdout, output)
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
