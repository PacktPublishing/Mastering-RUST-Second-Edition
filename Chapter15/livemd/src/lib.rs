// livemd/src/lib.rs

use wasm_bindgen::prelude::*;

use comrak::{markdown_to_html, ComrakOptions};

#[wasm_bindgen]
pub fn parse(source: &str) -> String {
    markdown_to_html(source, &ComrakOptions::default())
}
