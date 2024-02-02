mod cli;
mod args;

use async_trait::async_trait;
use cli::Cli;
use futures_util::{pin_mut, StreamExt};
use mdns::{Record, RecordKind};
use ractor::{concurrency::JoinHandle, Actor, ActorProcessingErr, ActorRef, OutputPort};
use std::{
    error::Error,
    fmt::Arguments,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::spawn;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Cli::run();

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

#[derive(Debug)]
struct Response {
    responded_at: Instant,
    response: mdns::Response,
}

impl Response {
    pub fn from_mdns_response(responded_at: Instant, response: mdns::Response) -> Self {
        Self {
            responded_at,
            response,
        }
    }
}

struct Scanner;

struct ScannerState {
    handle: JoinHandle<()>,
    //output: Arc<OutputPort<Arc<Response>>>,
}

#[async_trait]
impl Actor for Scanner {
    type Msg = ();

    type State = ScannerState;

    type Arguments = Arc<OutputPort<Arc<Response>>>;

    async fn pre_start(
        &self,
        actor: ActorRef<Self::Msg>,
        output: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        let handle = spawn(async move {
            let service_name = "_coap._udp.local";
            let stream = match mdns::discover::all(service_name, Duration::from_secs(5)) {
                Ok(stream) => stream,
                Err(e) => {
                    actor.stop(Some(format!(
                        "Failed to start mdns discovery: {}",
                        e.to_string()
                    )));
                    return;
                }
            };

            let stream = stream.listen();
            pin_mut!(stream);

            while let Some(Ok(mdns_response)) = stream.next().await {
                let response = Response::from_mdns_response(Instant::now(), mdns_response);
                output.send(response.into());
            }
        });

        Ok(ScannerState { handle })
    }

    async fn post_stop(
        &self,
        _: ActorRef<Self::Msg>,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        state.handle.abort();
        Ok(())
    }
}

struct StdoutPublisher;

#[async_trait]
impl Actor for StdoutPublisher {
    type Msg = Arc<Response>;

    type State = ();

    type Arguments = Arc<OutputPort<Arc<Response>>>;

    async fn pre_start(
        &self,
        actor: ActorRef<Self::Msg>,
        output: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        output.subscribe(actor, Some);
        Ok(())
    }

    async fn handle(
        &self,
        _: ActorRef<Self::Msg>,
        message: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        println!("From stdout publisher: {message:?}");
        Ok(())
    }
}
