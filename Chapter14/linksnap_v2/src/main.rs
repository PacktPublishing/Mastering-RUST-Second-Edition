// linksnap_v2/src/main.rs

use actix_web::{http, server, App};
use actix_web::middleware::Logger;
use actix::SyncArbiter;
use std::env;
use log::info;
use dotenv::dotenv;

mod schema;
mod models;
mod route_handlers;
mod state;
use crate::route_handlers::{index, links, add_link, rm_link};
use crate::state::State;

#[macro_use]
extern crate diesel;

fn init_env() {
    dotenv().ok();
    env::set_var("RUST_LOG", "linksnap_v2=info");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    info!("Starting http server: 127.0.0.1:8080");
}

fn main() {
    init_env();
    let system = actix::System::new("linksnap");
    let state = State::init();

    let web_app = move || {
        App::with_state(state.clone())
            .middleware(Logger::default())
            .route("/", http::Method::GET, index)
            .route("/links", http::Method::GET, links)
            .route("/add", http::Method::POST, add_link)
            .route("/rm", http::Method::DELETE, rm_link)
    };

    server::new(web_app).bind("127.0.0.1:8080").unwrap().start();
    let _ = system.run();
}
