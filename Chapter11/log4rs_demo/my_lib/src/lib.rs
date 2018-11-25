// log4rs_demo/my_lib/lib.rs

#[macro_use]
extern crate log;

pub struct Config;

impl Config {
    pub fn load_global_config() {
        debug!("Configuration files loaded");
    }
}