// hews/src/main.rs

mod app;
mod hackernews;

use crate::app::App;

fn main() {
    let (app, rx) = App::new();
    app.launch(rx);
}
