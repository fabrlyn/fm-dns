use clap::{command, Parser};

use crate::args::ServiceQuery;

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
    pub fn run() {
        let cli = Cli::parse();
        println!("Service query: {:?}", cli.service_query);
    }
}

fn parse_service_query(input: &str) -> Result<ServiceQuery, String> {
    ServiceQuery::decode(input)
        .ok_or("Invalid service query, needs to in the format: _service._proto.domain".to_string())
}
