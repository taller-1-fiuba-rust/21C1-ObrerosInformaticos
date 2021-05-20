use crate::threadpool::ThreadPool;
use crate::request::Request;
use std::net::TcpListener;
use std::io::prelude::*;
use std::io::BufReader;
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

    fn handle_connection(stream: TcpStream) {

        /*
         * Redis supports strings up to 512mb of size. Therefore we feed the parser line by line in order
         * to avoid storing all the strings.
         */
        let mut request = Request::new();
        let mut line = String::new();
        let mut reader = BufReader::new(stream.try_clone().unwrap());

        while reader.read_line(&mut line).unwrap() > 0 {
            request.feed(&line);
        }

        let _command = request.build();
        // esto desp
        // let result = execute(request.command);
        // let response = Response::new(result);
        // let response_str = response.serialize();
        // stream.write...
    }
}
