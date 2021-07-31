use crate::http;
use crate::server::THREADS;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use threadpool::threadpool::ThreadPool;

/// Struct which listens for connections and executes the given commands.
pub struct Listener {
    pool: ThreadPool,
    addr: String,
}

impl Listener {
    pub fn new(addr: String) -> Self {
        let pool = ThreadPool::new(THREADS);
        Listener { pool, addr }
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
        println!("Try REDIS WEBSERVER started on address '{}'...", self.addr);

        for stream in listener.incoming() {
            let socket = stream.unwrap();
            self.pool.spawn(move || {
                let result = Listener::handle_connection(socket);
                if let Err(e) = result {
                    println!("Error whilst parsing request:\n {}", e);
                }
            });
        }
    }

    fn handle_connection(mut socket: TcpStream) -> Result<String, &'static str> {
        let request_str = Listener::read_request_string(&mut socket)?;
        if request_str.is_empty() {
            return Ok(String::new());
        }
        println!("Received HTTP request");
        let request = http::request::Request::parse(request_str)?;

        println!("{}", request.to_string());
        Ok(String::new())
    }

    fn read_request_string(stream: &mut TcpStream) -> Result<String, &'static str> {
        let mut contents = Vec::new();
        let mut buffer = [0; 512];
        stream
            .set_read_timeout(Some(Duration::from_millis(10)))
            .ok()
            .ok_or("Failed to read from socket")?;
        while let Ok(r) = stream.read(&mut buffer) {
            if r == 0 {
                break;
            }
            contents.extend_from_slice(&buffer[0..r]);
        }
        String::from_utf8(contents).ok().ok_or("Bad request")
    }
}
