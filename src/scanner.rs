use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use futures_util::{pin_mut, StreamExt};
use ractor::{Actor, ActorProcessingErr, ActorRef, OutputPort};
use serde::{Deserialize, Serialize};
use tokio::{spawn, task::JoinHandle};

use crate::args::ServiceQuery;

#[derive(Debug, Serialize)]
pub struct Response {
    responded_at: chrono::DateTime<Utc>,
    //response: mdns::Response,
}

impl Response {
    pub fn from_mdns_response(responded_at: DateTime<Utc>, response: mdns::Response) -> Self {
        Self {
            responded_at,
            //response,
        }
    }
}

pub struct Arguments {
    pub output_port: Arc<OutputPort<Arc<Response>>>,
    pub service_query: ServiceQuery,
}

pub struct Scanner;

pub struct ScannerState {
    handle: JoinHandle<()>,
}

#[async_trait]
impl Actor for Scanner {
    type Msg = ();

    type State = ScannerState;

    type Arguments = Arguments;

    async fn pre_start(
        &self,
        actor: ActorRef<Self::Msg>,
        arguments: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        let handle = spawn(async move {
            let stream = match mdns::discover::all(
                arguments.service_query.to_string(),
                Duration::from_secs(5),
            ) {
                Ok(stream) => stream,
                Err(e) => {
                    actor.stop(Some(format!("Failed to start mdns discovery: {}", e)));
                    return;
                }
            };

            let stream = stream.listen();
            pin_mut!(stream);

            while let Some(Ok(mdns_response)) = stream.next().await {
                let response = Response::from_mdns_response(Utc::now(), mdns_response);
                arguments.output_port.send(response.into());
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
