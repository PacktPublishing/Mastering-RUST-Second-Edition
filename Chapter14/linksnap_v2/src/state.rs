// linksnap_v2/src/state.rs

use crate::models::{Link, LinkId};
use crate::schema::linksnap;
use actix::Addr;
use actix::SyncArbiter;
use actix::{Handler, Message};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
use serde_derive::{Deserialize, Serialize};
use std::env;
use std::ops::Deref;

const DB_THREADS: usize = 3;

use actix::Actor;
use actix::SyncContext;
use actix_web::{error, Error};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;
pub struct Db(pub PgPool);

impl Actor for Db {
    type Context = SyncContext<Self>;
}

impl Db {
    pub fn get_conn(&self) -> Result<PgPooledConnection, Error> {
        self.0.get().map_err(|e| error::ErrorInternalServerError(e))
    }
}

pub fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

#[derive(Clone)]
pub struct State {
    pub inner: Addr<Db>,
}

impl State {
    pub fn init() -> State {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = init_pool(&database_url).expect("Failed to create pool");
        let addr = SyncArbiter::start(DB_THREADS, move || Db(pool.clone()));
        let state = State {
            inner: addr.clone(),
        };
        state
    }
    pub fn get(&self) -> &Addr<Db> {
        &self.inner
    }
}

pub struct GetLinks;

impl Message for GetLinks {
    type Result = Result<Vec<Link>, Error>;
}

impl Handler<GetLinks> for Db {
    type Result = Result<Vec<Link>, Error>;

    fn handle(&mut self, _new_link: GetLinks, _: &mut Self::Context) -> Self::Result {
        Link::get_links(self.get_conn()?.deref())
            .map_err(|_| error::ErrorInternalServerError("Failed to retrieve links"))
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[derive(Insertable)]
#[table_name = "linksnap"]
pub struct AddLink {
    pub title: String,
    pub url: String,
}

impl Message for AddLink {
    type Result = Result<(), Error>;
}

impl Handler<AddLink> for Db {
    type Result = Result<(), Error>;

    fn handle(&mut self, new_link: AddLink, _: &mut Self::Context) -> Self::Result {
        Link::add_link(new_link, self.get_conn()?.deref())
            .map(|_| ())
            .map_err(|_| error::ErrorInternalServerError("Failed inserting link"))
    }
}

#[derive(Serialize, Deserialize)]
pub struct RmLink {
    pub id: LinkId,
}

impl Message for RmLink {
    type Result = Result<usize, Error>;
}

impl Handler<RmLink> for Db {
    type Result = Result<usize, Error>;
    fn handle(&mut self, link: RmLink, _: &mut Self::Context) -> Self::Result {
        let db_ref = self.get_conn()?;
        Link::rm_link(link.id, db_ref.deref())
            .map_err(|_| error::ErrorInternalServerError("Failed to remove links"))
    }
}
