use crate::scanner::model::Response;
use async_trait::async_trait;
use ractor::{ActorProcessingErr, ActorRef};
use std::sync::Arc;

pub type Actor = ActorRef<Msg>;

pub type ActorResult<T> = Result<T, ActorProcessingErr>;

pub type Arguments = ();

pub type Msg = Arc<Response>;

pub type State = ();

pub struct Stdout;

#[async_trait]
impl ractor::Actor for Stdout {
    type Arguments = Arguments;
    type Msg = Msg;
    type State = State;

    async fn pre_start(&self, _: Actor, _: Arguments) -> ActorResult<State> {
        Ok(())
    }

    async fn handle(&self, _: Actor, message: Msg, _: &mut State) -> ActorResult<()> {
        handle_message(message);
        Ok(())
    }
}

fn handle_message(message: Msg) {
    match serde_json::to_string(message.as_ref()) {
        Ok(message) => println!("{}", message),
        Err(e) => eprintln!("Failed to serialize message: {e:?}"),
    };
}
