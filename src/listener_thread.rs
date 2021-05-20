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
        println!("REDIS server started on address '{}'...", self.addr);
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            self.pool.spawn(|| {
                ListenerThread::handle_connection(stream);
            });
        }
    }

    fn handle_connection(mut stream: TcpStream) {
        let mut request = Request::new();
        let reader = BufReader::new(stream.try_clone().unwrap());

        for line in reader.lines() {
            let l = line.unwrap();
            println!("Message '{}'", &l);
            let formatted = format!("{}\r\n", &l);
            match request.feed(&formatted) {
                Err(e) => println!("{}", e),
                _ => {}
            }
        }

        let command = request.build();

        println!("Recieved command '{} {}'", command.name(), command.arguments().iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));

        let response = ResponseBuilder::new();
        // execute_command(request.command, &mut response);
        let response_str = response.serialize();

        stream.write_all(response_str.as_bytes()).unwrap();
    }
}
