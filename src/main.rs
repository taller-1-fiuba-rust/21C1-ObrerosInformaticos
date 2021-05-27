use crate::config::configuration::Configuration;
use crate::logging::logger::Logger;
mod logging;
use std::env;
mod config;
mod execution;
mod listener_thread;
mod protocol;
mod server;
mod server_command;
mod storage;
mod threadpool;

fn main() {
    let args: Vec<String> = env::args().collect();
    let logger = Logger::new();
    let mut configuration = Configuration::new(logger);

    if args.len() > 1 {
        if let Err(msj) = configuration.set_config(&args[1]) {
            println!("{}", msj);
            return;
        }
    }

    let mut server = server::Server::new(configuration);
    server.run();
    server.join();
}
