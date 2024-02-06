pub mod model;
use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use chrono::Utc;
use futures_util::{pin_mut, StreamExt};
use mdns::discover::{self, Discovery};
use ractor::{ActorProcessingErr, ActorRef, OutputPort};

use tokio::{spawn, task::JoinHandle};

use crate::args::ServiceQuery;

use self::model::Response;

pub type Actor = ActorRef<Msg>;

pub type ActorResult<T> = Result<T, ActorProcessingErr>;

pub type Port = Arc<OutputPort<Arc<Response>>>;

pub struct Arguments {
    pub port: Port,
    pub service_query: ServiceQuery,
    pub interval: Duration,
}

pub type Msg = ();

pub struct State {
    handle: JoinHandle<Result<(), ()>>,
}

pub struct Scanner;

#[async_trait]
impl ractor::Actor for Scanner {
    type Arguments = Arguments;
    type Msg = ();
    type State = State;

    async fn pre_start(&self, actor: Actor, arguments: Arguments) -> ActorResult<State> {
        Ok(State {
            handle: start_discovery(actor, arguments),
        })
    }

    async fn post_stop(&self, _: Actor, state: &mut State) -> ActorResult<()> {
        state.handle.abort();
        Ok(())
    }
}

fn start_discovery(actor: Actor, arguments: Arguments) -> JoinHandle<Result<(), ()>> {
    spawn(discover(actor, arguments))
}

async fn discover(actor: Actor, arguments: Arguments) -> Result<(), ()> {
    let discovery = create_discovery(&actor, &arguments)?;
    process_discovery(&actor, &arguments, discovery).await
}

fn create_discovery(actor: &Actor, arguments: &Arguments) -> Result<Discovery, ()> {
    let service_query = arguments.service_query.to_string();
    let interval = arguments.interval;

    discover::all(service_query, interval).map_err(|e| {
        actor.stop(Some(format!("Failed to start mdns discovery: {}", e)));
        
    })
}

async fn process_discovery(
    actor: &Actor,
    arguments: &Arguments,
    discovery: Discovery,
) -> Result<(), ()> {
    let stream = discovery.listen();
    pin_mut!(stream);

    while let Some(Ok(mdns_response)) = stream.next().await {
        let response = Response::from_mdns_response(Utc::now(), mdns_response);
        arguments.port.send(response.into());
    }

    actor.stop(None);
    Ok(())
}
