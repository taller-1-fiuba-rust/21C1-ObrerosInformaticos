use crate::server::THREADS;
use crate::threadpool::ThreadPool;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

/// Struct which listens for connections and executes the given commands.
pub struct Listener {
    pool: ThreadPool,
    addr: String,
}

impl Listener {
    pub fn new(addr: String) -> Self {
        let pool = ThreadPool::new(THREADS);
        Listener {
            pool,
            addr,
        }
    }

    pub fn run(&self) {
        println!("Trying to bind on address {}", self.addr);
        let listener = match TcpListener::bind(&self.addr) {
            Ok(s) => s,
            Err(e) => {
                println!("Failed to bind to socket with error: '{}'", e);
                panic!("{}", e);
            }
        };
        println!(
            "Try REDIS WEBSERVER started on address '{}'...",
            self.addr
        );

        for stream in listener.incoming() {
            let socket = stream.unwrap();
            self.pool.spawn(move || {
                Listener::handle_connection(socket);
            });
        }
    }

    fn handle_connection(mut socket: TcpStream) {
        let mut buffer = String::new();

        let size = socket.read_to_string(&mut buffer).unwrap();
        println!("{}", buffer);
    }
}
