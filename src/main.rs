use std::env;
mod config;
mod execution;
mod listener_thread;
mod protocol;
mod server;
mod storage;
mod threadpool;
use crate::config::configuration::Configuration;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut configuration = Configuration::new();

    if args.len() > 1 {
        if let Err(msj) = configuration.set_config(&args[1]) {
            println!("{}", msj);
            return;
        }
    }

    let addr = "127.0.0.1:6379".to_string();
    let mut server = server::Server::new(addr);
    server.run();
    server.join();
}
