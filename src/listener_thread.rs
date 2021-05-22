use crate::execution::Execution;
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
        let mut result: Result<bool, String> = Err("Empty message".to_string());

        for line in reader.lines() {
            let l = line.unwrap();
            let formatted = format!("{}\r\n", &l);
            result = request.feed(&formatted);
            if let Ok(val) = result {
                if val {
                    break;
                } else {
                }
            } else if result.is_err() {
                break;
            }
        }
        if let Err(e) = result {
            println!("{}", e);
            return;
        }

        let command = request.build();

        println!(
            "Received command '{} {}'",
            command.name(),
            command
                .arguments()
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );

        let mut response = ResponseBuilder::new();
        if let Err(e) = Execution::run(&command, &mut response) {
            println!("{}", e);
        }
        let response_str = response.serialize();
        stream.write_all(response_str.as_bytes()).unwrap();
    }
}
