// hyperurl/src/service.rs

use std::sync::RwLock;
use std::collections::HashMap;
use std::sync::{Arc};
use std::str;
use hyper::Request;
use hyper::{Body, Response};
use hyper::rt::{Future, Stream};
use hyper::Method;
use hyper::StatusCode;

use lazy_static::lazy_static;
use log::info;

use crate::shortener::shorten_url;

use futures::future;

use crate::index::INDEX_PAGE;
use crate::LISTEN_ADDR;

type UrlDb = Arc<RwLock<HashMap<String, String>>>;
type BoxFut = Box<Future<Item = Response<Body>, Error = hyper::Error> + Send>;

lazy_static! {
    static ref SHORT_URLS: UrlDb = Arc::new(RwLock::new(HashMap::new()));
}

pub(crate) fn url_service(req: Request<Body>) -> BoxFut {
    
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            let reply = Response::new(Body::from(format!("{}",INDEX_PAGE)));
            Box::new(future::ok(reply))
        }
        (&Method::GET, _) => {
            let url_db = SHORT_URLS.read().unwrap();
            let uri = req.uri().to_string();
            let short_url = format!("{}{}", LISTEN_ADDR, uri);
            let long_url = url_db.get(&short_url);
            println!("LONG URL {:?}",long_url);
            let reply = match long_url {
                Some(url) => {
                    info!("Redirecting from {} to {}", short_url, url);
                    Response::builder().status(StatusCode::TEMPORARY_REDIRECT)
                    .header("Location", url.to_string()).body(Body::empty()).unwrap()
                }
                None => {
                    Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::from(format!("Bad URI: {:?}", uri))).unwrap()
                }
            };
            Box::new(future::ok(reply))
        }
        (&Method::POST, "/shorten") => {
            info!("Received request from : {:?}", req.headers());
            let reply = req.into_body().concat2().map(move |chunk| {
                let c = chunk.iter().cloned().collect::<Vec<u8>>();
                let url_to_shorten = str::from_utf8(&c).unwrap();
                let short_url = shorten_url(url_to_shorten);
                let reply = Response::new(Body::from(format!("{}", &short_url)));
                SHORT_URLS.write().unwrap().insert(short_url, url_to_shorten.to_string());
                reply
            });
            Box::new(reply)
        }
        (_, _) => {
            let no_route = Response::new(Body::from("No route handler for this path"));
            Box::new(future::ok(no_route))
        }
    }
}
