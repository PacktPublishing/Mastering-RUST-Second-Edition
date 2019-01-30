// linksnap_v2/src/models.rs

use chrono::prelude::*;
use diesel::prelude::Queryable;
use chrono::NaiveDateTime;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use crate::state::AddLink;
use crate::schema::linksnap;
use crate::schema::linksnap::dsl::{linksnap as get_links};
use serde_derive::{Serialize, Deserialize};

pub type LinkId = i32;

#[derive(Queryable, Debug)]
pub struct Link {
    pub id: i32,
    pub title: String,
    pub url: String,
    pub added: NaiveDateTime,
}

impl Link {
    pub fn add_link(new_link: AddLink, conn: &PgConnection) -> QueryResult<usize> {
        diesel::insert_into(linksnap::table)
            .values(&new_link)
            .execute(conn)
    }

    pub fn get_links(conn: &PgConnection) -> QueryResult<Vec<Link>> {
        get_links.order(linksnap::id.desc()).load::<Link>(conn)
    }

    pub fn rm_link(id: LinkId, conn: &PgConnection) -> QueryResult<usize> {
        diesel::delete(get_links.find(id)).execute(conn)
    }
}
