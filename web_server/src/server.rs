use crate::listener::Listener;
use std::thread;
use std::thread::JoinHandle;
use redis_server::server::Server as RedisServer;

pub const THREADS: usize = 32;

#[allow(dead_code)]
/// A server struct
pub struct Server {
    addr: String,
    port: u16,
    handle: Option<JoinHandle<()>>,
    redis_sv: RedisServer,
    redis_port: u16,
}

impl Server {
    pub fn new(addr: &str, port: u16, redis_sv: RedisServer, redis_port: u16) -> Self {
        Server {
            addr: addr.to_string(),
            port,
            handle: None,
            redis_sv,
            redis_port,
        }   
    }

    pub fn run(&mut self) {
        let addr_and_port = self.get_addr_and_port();
        let redis_port = self.redis_port.clone();
        let handle = thread::spawn(move || {
            let listener = Listener::new(addr_and_port, redis_port);
            listener.run();
        });
        self.handle = Some(handle);
    }

    /// Returns the joined address and port
    fn get_addr_and_port(&self) -> String {
        self.addr.clone() + ":" + &self.port.to_string()
    }

    /// Waits for the server to finish executing
    pub fn join(&mut self) {
        if self.handle.is_none() {
            panic!("Server was joined before ran.");
        }
        self.handle.take().unwrap().join().unwrap();
    }
}
