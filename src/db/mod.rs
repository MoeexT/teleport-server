use std::collections::HashSet;

use crate::clip::{ClipType, Ttl};

pub mod entity;
pub mod mysql_store;
pub mod redis_store;

pub struct User {
    key: String,
    secret: String,
    description: String,
    ttl: Ttl,
    clip_authorities: HashSet<ClipType>,
}

impl User {
    pub fn new(
        key: &str,
        secret: &str,
        description: &str,
        ttl: Ttl,
        clip_authorities: HashSet<ClipType>,
    ) -> Self {
        Self {
            key: key.to_owned(),
            secret: secret.to_owned(),
            description: description.to_owned(),
            ttl,
            clip_authorities,
        }
    }
}
