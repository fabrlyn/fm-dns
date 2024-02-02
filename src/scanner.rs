use std::{
    net::{Ipv4Addr, Ipv6Addr},
    sync::Arc,
    time::Duration,
};

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use futures_util::{pin_mut, StreamExt};
use ractor::{Actor, ActorProcessingErr, ActorRef, OutputPort};
use serde::Serialize;
use tokio::{spawn, task::JoinHandle};

use crate::args::ServiceQuery;

#[derive(Debug, Serialize)]
pub struct Response {
    pub additional: Vec<Record>,
    pub answers: Vec<Record>,
    pub nameservers: Vec<Record>,
    pub responded_at: chrono::DateTime<Utc>,
}

/// The CLASS value according to RFC 1035
#[derive(Debug, Clone, Serialize)]
pub enum Class {
    /// the Internet
    IN = 1,
    /// the CSNET class (Obsolete - used only for examples in some obsolete
    /// RFCs)
    CS = 2,
    /// the CHAOS class
    CH = 3,
    /// Hesiod [Dyer 87]
    HS = 4,
}

#[derive(Debug, Clone, Serialize)]
pub struct Record {
    pub name: String,
    pub class: Class,
    pub ttl: u32,
    pub kind: RecordKind,
}

#[derive(Debug, Clone, Serialize)]
pub enum RecordKind {
    A(Ipv4Addr),
    AAAA(Ipv6Addr),
    CNAME(String),
    MX {
        preference: u16,
        exchange: String,
    },
    NS(String),
    SRV {
        priority: u16,
        weight: u16,
        port: u16,
        target: String,
    },
    TXT(Vec<String>),
    PTR(String),
    /// A record kind that hasn't been implemented by this library yet.
    Unimplemented(Vec<u8>),
}

impl From<mdns::Record> for Record {
    fn from(value: mdns::Record) -> Self {
        Self {
            name: value.name,
            class: value.class.into(),
            ttl: value.ttl,
            kind: value.kind.into(),
        }
    }
}

impl From<dns_parser::Class> for Class {
    fn from(value: dns_parser::Class) -> Self {
        match value {
            dns_parser::Class::IN => Self::IN,
            dns_parser::Class::CS => Self::CS,
            dns_parser::Class::CH => Self::CH,
            dns_parser::Class::HS => Self::HS,
        }
    }
}

impl From<mdns::RecordKind> for RecordKind {
    fn from(value: mdns::RecordKind) -> Self {
        match value {
            mdns::RecordKind::A(r) => Self::A(r),
            mdns::RecordKind::AAAA(r) => Self::AAAA(r),
            mdns::RecordKind::CNAME(r) => Self::CNAME(r),
            mdns::RecordKind::MX {
                preference,
                exchange,
            } => Self::MX {
                preference,
                exchange,
            },
            mdns::RecordKind::NS(r) => Self::NS(r),
            mdns::RecordKind::SRV {
                priority,
                weight,
                port,
                target,
            } => Self::SRV {
                priority,
                weight,
                port,
                target,
            },
            mdns::RecordKind::TXT(r) => Self::TXT(r),
            mdns::RecordKind::PTR(r) => Self::PTR(r),
            mdns::RecordKind::Unimplemented(r) => Self::Unimplemented(r),
        }
    }
}

impl Response {
    pub fn from_mdns_response(responded_at: DateTime<Utc>, response: mdns::Response) -> Self {
        Self {
            responded_at,
            additional: response.additional.into_iter().map(Into::into).collect(),
            answers: response.answers.into_iter().map(Into::into).collect(),
            nameservers: response.nameservers.into_iter().map(Into::into).collect(),
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
