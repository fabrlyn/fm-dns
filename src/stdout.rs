use std::sync::Arc;

use async_trait::async_trait;
use ractor::{Actor, ActorProcessingErr, ActorRef, OutputPort};

use crate::scanner::model::Response;

pub struct Stdout;

pub type Msg = Arc<Response>;

pub type Arguments = Arc<OutputPort<Arc<Response>>>;

#[async_trait]
impl Actor for Stdout {
    type Msg = Msg;

    type State = ();

    type Arguments = Arguments;

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
        _state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match serde_json::to_string(message.as_ref()) {
            Ok(message) => println!("{}", message),
            Err(e) => eprintln!("Failed to serialize message: {e:?}"),
        };

        Ok(())
    }
}
