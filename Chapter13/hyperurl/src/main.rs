// hyperurl/src/main.rs

use log::{info, error};
use std::env;
use hyper::Server;
use hyper::service::service_fn;
use hyper::rt::{self, Future};

mod shortener;
mod service;
mod index;

use crate::service::url_service;

const LISTEN_ADDR: &str = "127.0.0.1:3002";

fn main() {
    env::set_var("RUST_LOG","hyperurl=info");
    pretty_env_logger::init();
    let addr = LISTEN_ADDR.parse().unwrap();

    let server = Server::bind(&addr)
        .serve(|| service_fn(url_service))
        .map_err(|e| error!("server error: {}", e));
    info!("hyperurl is listening at {}", addr);

    rt::run(server);
}
