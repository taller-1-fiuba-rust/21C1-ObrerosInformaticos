use crate::config::configuration::Configuration;
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
mod string_command;
mod threadpool;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut configuration = Configuration::new();

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
