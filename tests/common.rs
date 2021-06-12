use proyecto_taller_1::server::Server;
use proyecto_taller_1::config::configuration::Configuration;
use redis::{Client, FromRedisValue};

const PORT: u16 = 10001;

pub fn setup() -> (Server, Client) {
    let mut config = Configuration::new();
    config.set_port(PORT);
    let mut sv = Server::new(config);
    sv.run();
    let client = redis::Client::open(format!("redis://127.0.0.1:{}/", PORT)).unwrap();
    return (sv, client);
}

pub fn query<T: FromRedisValue>(client: & Client, cmd: &'static str, args: &[&str]) -> T {
    let mut cmd = redis::cmd(cmd);
    for arg in args {
        cmd.arg(*arg);
    }
    cmd.query(&mut client.get_connection().unwrap()).unwrap()
}