use crate::server::THREADS;
use threadpool::threadpool::ThreadPool;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::str::Utf8Error;
use crate::http;

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
        let request_result = Listener::read_request_string(&mut socket);
        if let Err(_) = request_result {
            // write invalid header
            return;
        }
        let request = Request::new(request_result.unwrap());
        println!("{}", request_str);
    }

    fn read_request_string(stream: &mut TcpStream) -> Result<String, &'static str> {
        let mut contents = Vec::new();
        let mut buffer = [0;512];
        stream.set_read_timeout(Some(Duration::from_millis(10)));
        loop {
            match stream.read(&mut buffer) {
                Ok(r) => {
                    if r == 0 {
                        break
                    }
                    contents.extend_from_slice(&buffer[0..r]);
                }
                Err(_) => {
                    /*
                    if !contents.is_empty() {
                        break;
                    }*/
                    break
                }
            }
        }
        String::from_utf8(contents).ok().ok_or("Bad request")
    }
}
