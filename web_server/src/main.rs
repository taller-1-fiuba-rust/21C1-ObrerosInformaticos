mod client;
mod http;
mod listener;
mod request_handler;
mod server;
use redis_server::config::configuration::Configuration;
use redis_server::logging::logger::Logger;
use redis_server::server::Server;
use std::sync::Arc;

const REDIS_PORT: u16 = 10003;

fn main() {
    let mut config = Configuration::new();
    let logger: Arc<Logger> = Arc::new(Logger::new(config.get_logfile()).unwrap());
    config.set_port(REDIS_PORT);
    let mut redis_sv = Server::new(config, logger);
    redis_sv.run();

    let mut sv = server::Server::new("localhost", 8080, redis_sv, REDIS_PORT);
    sv.run();
    sv.join();
}
