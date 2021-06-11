use proyecto_taller_1::server::Server;
use proyecto_taller_1::config::configuration::Configuration;
use redis::Connection;

pub fn setup() -> (Server, Connection) {
    let config = Configuration::new();
    let mut sv = Server::new(config);
    sv.run();
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    return (sv, client.get_connection().unwrap());
}