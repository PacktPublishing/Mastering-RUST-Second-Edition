// linksnap_v2/src/route_handlers.rs

use actix_web::{Error, HttpRequest, HttpResponse};

use actix_web::AsyncResponder;
use actix_web::HttpMessage;
use actix_web::Query;
use futures::Future;
use crate::state::{State, AddLink, GetLinks, RmLink};
use actix_web::FromRequest;

use serde_derive::{Deserialize, Serialize};

type ResponseFuture = Box<Future<Item = HttpResponse, Error = Error>>;

macro_rules! server_err {
    ($msg:expr) => {
        Err(actix_web::error::ErrorInternalServerError($msg))
    };
}

pub fn index(_req: HttpRequest<State>) -> HttpResponse {
    HttpResponse::from("Welcome to Readlist API server")
}

pub fn add_link(req: HttpRequest<State>) -> ResponseFuture {
    req.json()
        .from_err()
        .and_then(move |link: AddLink| {
            let state = req.state().get();
            state.send(link).from_err().and_then(|e| match e {
                Ok(_) => Ok(HttpResponse::Ok().finish()),
                Err(_) => server_err!("Failed to add link"),
            })
        })
        .responder()
}

pub fn links(req: HttpRequest<State>) -> ResponseFuture {
    let state = &req.state().get();
    state
        .send(GetLinks)
        .from_err()
        .and_then(|e| match e {
            Ok(e) => Ok(HttpResponse::Ok().body(format!("{:?}", e))),
            Err(_) => server_err!("Failed to retrieve links"),
        })
        .responder()
}

pub fn rm_link(req: HttpRequest<State>) -> ResponseFuture {
    let params: Query<RmLink> = Query::extract(&req).unwrap();
    let state = &req.state().get();
    state
        .send(RmLink { id: params.id })
        .from_err()
        .and_then(|e| match e {
            Ok(e) => Ok(HttpResponse::Ok().body(format!("{}",  e))),
            Err(_) => server_err!("Failed to remove link"),
        })
        .responder()
}
