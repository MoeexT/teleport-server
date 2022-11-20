use std::collections::HashSet;

use deku::prelude::*;
use teleport_server::clip::{ClipType, Ttl};
use teleport_server::util::deku_custom::{string_reader, string_writer};

pub mod entity;
pub mod mysql_store;
pub mod redis_store;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct User {
    #[deku(update = "self.key.as_bytes().len()")]
    l_key: u8,
    #[deku(update = "self.secret.as_bytes().len()")]
    l_sct: u8,
    #[deku(update = "self.description.as_bytes().len()")]
    l_dsc: u8,
    #[deku(update = "self.clip_authorities.len()")]
    l_cat: u8,

    #[deku(
        reader = "string_reader(deku::rest, *l_key)",
        writer = "string_writer(deku::output, &self.key)"
    )]
    pub key: String,

    #[deku(
        reader = "string_reader(deku::rest, *l_sct)",
        writer = "string_writer(deku::output, &self.secret)"
    )]
    secret: String,

    #[deku(
        reader = "string_reader(deku::rest, *l_dsc)",
        writer = "string_writer(deku::output, &self.description)"
    )]
    pub description: String,

    pub ttl: Ttl,
    #[deku(count = "l_cat")]
    pub clip_authorities: HashSet<ClipType>,
}

impl User {
    pub fn new(
        key: &str,
        secret: &str,
        description: &str,
        ttl: Ttl,
        clip_authorities: HashSet<ClipType>,
    ) -> Self {
        let mut usr = Self {
            l_key: 0,
            l_sct: 0,
            l_dsc: 0,
            l_cat: 0,
            key: String::from(key),
            secret: String::from(secret),
            description: String::from(description),
            ttl,
            clip_authorities,
        };
        let _ = usr.update();

        usr
    }
}
