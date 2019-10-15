extern crate actix_pythoneer;

use std::env;
use std::sync::{Arc, Mutex};

use actix_pythoneer::config;
//use actix_pythoneer::io_tools;
use actix_pythoneer::server::run_server;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "--setup" => {
                config::setup();
                return;
            }
            _ => {
                println!("Unknown argument, exiting");
                return;
            }
        }
    }
    let config = Arc::new(Mutex::new(config::read_config().unwrap()));
    let _handler = run_server(config);
}
