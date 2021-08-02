mod http;
mod listener;
mod request_handler;
mod server;
mod client;
use redis_server::server::Server;
use redis_server::config::configuration::Configuration;
use redis_server::logging::logger::Logger;
use std::sync::Arc;
use std::sync::atomic::{AtomicU16, Ordering};

static PORT: AtomicU16 = AtomicU16::new(10002);

fn main() {
    let port = PORT.fetch_add(1, Ordering::SeqCst);
    let mut config = Configuration::new();
    let logger: Arc<Logger> = Arc::new(Logger::new(config.get_logfile()).unwrap());
    config.set_port(port);
    let mut redis_sv = Server::new(config, logger);
    redis_sv.run();

    let mut sv = server::Server::new("localhost", 8080, redis_sv, port);
    sv.run();
    sv.join();
}
