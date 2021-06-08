use crate::config::configuration::Configuration;
use crate::logging::logger::Logger;
use std::sync::{Arc, Mutex};
mod logging;
use std::env;
mod config;
mod execution;
mod key_command;
mod listener_thread;
mod protocol;
mod pubsub;
mod server;
mod server_command;
mod storage;
mod threadpool;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut configuration = Configuration::new();
    let logger = Logger::new(configuration.get_logfile()).unwrap();
    let logger_ref = Arc::new(Mutex::new(logger));

    if args.len() > 1 {
        if let Err(msj) = configuration.set_config(&args[1]) {
            println!("{}", msj);
            return;
        }
    }

    let mut server = server::Server::new(configuration, logger_ref.clone());
    server.run();
    server.join();
}
