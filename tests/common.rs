use proyecto_taller_1::config::configuration::Configuration;
use proyecto_taller_1::logging::logger::Logger;
use proyecto_taller_1::server::Server;
use redis::{Client, FromRedisValue};
use std::sync::atomic::{AtomicU16, Ordering};
use std::sync::Arc;
use std::time::SystemTime;

static PORT: AtomicU16 = AtomicU16::new(10001);
const TIMEOUT: u64 = 5;

pub fn setup() -> (Server, Client) {
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
    let client = redis::Client::open(format!("redis://127.0.0.1:{}/", port)).unwrap();
    return (sv, client);
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
