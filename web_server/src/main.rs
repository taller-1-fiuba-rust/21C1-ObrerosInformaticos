mod http;
mod listener;
mod request_handler;
mod server;
mod client;
use redis_server::server::Server;
use redis_server::config::configuration::Configuration;
use redis_server::logging::logger::Logger;
use std::sync::Arc;

static PORT: u16 = 10010;

fn main() {
    let mut config = Configuration::new();
    let logger: Arc<Logger> = Arc::new(Logger::new(config.get_logfile()).unwrap());
    config.set_port(PORT);
    let mut redis_sv = Server::new(config, logger);
    redis_sv.run();

    let mut sv = server::Server::new("localhost", 8080, redis_sv);
    sv.run();
    sv.join();
}
