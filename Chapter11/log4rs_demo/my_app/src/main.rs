
#[macro_use]
extern crate log;
extern crate log4rs;

extern crate my_lib;

use my_lib::Config;

fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    error!("Sample app v{}", env!("CARGO_PKG_VERSION"));
    Config::load_global_config();
}
