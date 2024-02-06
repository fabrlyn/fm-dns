use clap::{command, Parser};
use fm_dns::{model::ServiceQuery, Config};
use std::{sync::Arc, time::Duration};

#[derive(Debug, Parser)]
#[command(
    about = "Query mdns services on the network",
    name = "fm-dns",
    author = "fabrlyn"
)]
struct Cli {
    #[arg(
        help = "The service to query for on the network. Example: _googlecast._tcp.local",
        value_parser = parse_service_query 
    )]
    service_query: Arc<ServiceQuery>,
    #[arg(help = "The interval to query the network.", default_value = "60", value_parser = parse_interval, long="interval")]
    interval: Duration,
}

pub async fn run() {
    let cli = Cli::parse();

    fm_dns::run(Config {
        interval: cli.interval,
        service_query: cli.service_query,
    })
    .await;
}

fn parse_service_query(input: &str) -> Result<Arc<ServiceQuery>, String> {
    ServiceQuery::decode(input)
        .map(Arc::new)
        .ok_or("Invalid service query, needs to in the format: _service._proto.domain".to_string())
}

fn parse_interval(input: &str) -> Result<Duration, String> {
    str::parse(input)
        .map(Duration::from_secs)
        .map_err(|_| "Not a valid interval. Needs to be a number of seconds.".to_string())
}
