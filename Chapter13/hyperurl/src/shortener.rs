// hyperurl/src/shortener.rs

use crypto::digest::Digest;
use crypto::sha2::Sha256;

use crate::LISTEN_ADDR;

pub(crate) fn shorten_url(url: &str) -> String {
    let mut sha = Sha256::new();
    sha.input_str(url);
    let mut s = sha.result_str();
    s.truncate(5);
    format!("{}/{}", LISTEN_ADDR, s)
}
