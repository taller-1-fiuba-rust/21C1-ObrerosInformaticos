use redis_server::config::configuration::Configuration;
use redis_server::logging::logger::Logger;
use redis_server::server::Server;
use redis::{Client, FromRedisValue};
use std::sync::atomic::{AtomicU16, Ordering};
use std::sync::Arc;
use std::time::SystemTime;

static PORT: AtomicU16 = AtomicU16::new(10001);
const TIMEOUT: u64 = 5;

pub fn setup_server() -> (Server, u16) {
    let port = PORT.fetch_add(1, Ordering::SeqCst);

    let mut config = Configuration::new();
    config.set_port(port);
    let logger: Arc<Logger> = Arc::new(Logger::new(config.get_logfile()).unwrap());

    let mut sv = Server::new(config, logger);
    println!("Opening server on port {}", port);
    sv.run();
    let start = SystemTime::now();
    while !sv.poll_running() {
        if SystemTime::now().duration_since(start).unwrap().as_secs() > TIMEOUT {
            panic!("Failed to start REDIS server");
        }
    }
    (sv, port)
}

#[allow(dead_code)]
pub fn setup() -> (Server, Client) {
    let (sv, port) = setup_server();
    let client = setup_client(port);
    (sv, client)
}

pub fn setup_client(port: u16) -> Client {
    redis::Client::open(format!("redis://127.0.0.1:{}/", port)).unwrap()
}

pub fn query_string<T: FromRedisValue>(client: &Client, cmd: &str) -> T {
    let args: Vec<&str> = cmd.split(" ").collect();
    query(client, args[0], &args[1..])
}

pub fn query<T: FromRedisValue>(client: &Client, cmd: &str, args: &[&str]) -> T {
    let mut cmd = redis::cmd(cmd);
    for arg in args {
        cmd.arg(*arg);
    }
    cmd.query(&mut client.get_connection().unwrap()).unwrap()
}
