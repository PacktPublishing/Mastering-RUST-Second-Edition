// shorten/src/main.rs

use quicli::prelude::*;
use structopt::StructOpt;
use reqwest::Client;

const HYPERURL_ADDR: &str = "127.0.0.1:3002";
const HTTP_PREFIX: &str = "http://";

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long = "url", short = "u")]
    url: String,
    #[structopt(flatten)]
    verbosity: Verbosity,
}

fn main() -> CliResult {
    let args = Cli::from_args();
    println!("Shortening: {}", args.url);
    let client = Client::new();
    let mut res = client
        .post(&format!("{}{}/shorten", HTTP_PREFIX, HYPERURL_ADDR))
        .body(args.url)
        .send()?;
    let a: String = res.text().unwrap();
    println!("{}{}", HTTP_PREFIX, a);
    Ok(())
}
