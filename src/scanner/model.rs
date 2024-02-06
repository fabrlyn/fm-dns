use std::net::{Ipv4Addr, Ipv6Addr};

use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Response {
    pub additional: Vec<Record>,
    pub answers: Vec<Record>,
    pub nameservers: Vec<Record>,
    pub responded_at: chrono::DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub enum Class {
    IN = 1,
    CS = 2,
    CH = 3,
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
