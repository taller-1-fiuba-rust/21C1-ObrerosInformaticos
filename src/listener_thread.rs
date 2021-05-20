use crate::protocol::request::Request;
use crate::protocol::response::ResponseBuilder;
use crate::threadpool::ThreadPool;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpListener;
use std::net::TcpStream;

pub struct ListenerThread {
    pool: ThreadPool,
    addr: String,
}

impl ListenerThread {
    pub fn new(addr: String) -> Self {
        let pool = ThreadPool::new(32);

        ListenerThread { pool, addr }
    }

    pub fn run(&self) {
        let listener = TcpListener::bind(&self.addr).unwrap();
        println!("Listening...");
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            self.pool.spawn(|| {
                ListenerThread::handle_connection(stream);
            });
        }
    }

    fn handle_connection(mut stream: TcpStream) {
        let mut request = Request::new();
        let mut line = String::new();
        let mut reader = BufReader::new(stream.try_clone().unwrap());

        while reader.read_line(&mut line).unwrap() > 0 {
            request.feed(&line);
        }

        let _command = request.build();
        let response = ResponseBuilder::new();
        // execute(request.command, &mut response);
        let response_str = response.serialize();
        stream.write_all(response_str.as_bytes()).unwrap();
    }
}
