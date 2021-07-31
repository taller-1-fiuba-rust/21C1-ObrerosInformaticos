use crate::config::configuration::Configuration;
use crate::logging::logger::Logger;
use std::env;
use std::sync::Arc;
mod client;
mod config;
mod execution;
mod key_command;
mod listener_thread;
mod lists_command;
mod logging;
mod monitor;
mod protocol;
mod pubsub;
mod pubsub_command;
mod server;
mod server_command;
mod set_command;
mod storage;
mod string_command;
mod threadpool;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut configuration = Configuration::new();
    let logger = Logger::new(configuration.get_logfile()).unwrap();
    let logger_ref = Arc::new(logger);

    if args.len() > 1 {
        if let Err(msj) = configuration.set_config(&args[1]) {
            println!("{}", msj);
            return;
        }
    }

    let mut server = server::Server::new(configuration, logger_ref);
    server.run();
    server.join();
}
