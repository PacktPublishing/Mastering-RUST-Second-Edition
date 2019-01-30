// linksnap/src/links.rs

use std::collections::HashMap;
use serde_derive::{Serialize, Deserialize};
use chrono::prelude::*;
use crate::state::AddLink;

pub type LinkId = i32;

#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    pub id: LinkId,
    pub title: String,
    pub url: String,
    pub added: String
}

#[derive(Serialize, Deserialize)]
pub struct Links {
    last_id: LinkId,
    links: HashMap<LinkId, Link>
}

impl Links {
    pub fn new() -> Self {
        Links { last_id: 0, links: HashMap::new() }
    }

    pub fn add_link(&mut self, add_link: AddLink) -> LinkId {
        let new_id = self.last_id + 1;
        self.last_id = new_id;
        let utc: DateTime<Utc> = Utc::now();
        self.links.insert(new_id, Link {
            id: new_id,
            title: add_link.title.to_string(),
            url: add_link.url.to_string(),
            added: utc.to_string()
        });
        new_id
    }

    pub fn rm_link(&mut self, id: LinkId) -> Option<LinkId> {
        self.links.remove(&id).map(|_| id)
    }

    pub fn links(&self) -> String {
        serde_json::to_string_pretty(&self.links).unwrap()
    }
}
