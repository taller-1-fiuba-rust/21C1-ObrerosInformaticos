use crate::execution::Execution;
use crate::protocol::request::Request;
use crate::protocol::response::ResponseBuilder;
use crate::threadpool::ThreadPool;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::Arc;

pub struct ListenerThread<'a> {
    pool: ThreadPool,
    addr: String,
    execution: Arc<&'a Execution<'a>>,
}

impl<'a> ListenerThread<'a> {
    pub fn new(addr: String, execution: &'a Execution) -> Self {
        let pool = ThreadPool::new(32);
        let exec = Arc::new(execution);

        ListenerThread { pool, addr, execution: exec }
    }

    pub fn run(&self) {
        let listener = TcpListener::bind(&self.addr).unwrap();
        println!("REDIS server started on address '{}'...", self.addr);
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let exec = self.execution.clone();
            self.pool.spawn(move || {
                ListenerThread::handle_connection(stream, exec);
            });
        }
    }

    fn handle_connection(mut stream: TcpStream, execution: Arc<&'a Execution>) {
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
        if let Err(e) = execution.run(&command, &mut response) {
            println!("{}", e);
        }
        let response_str = response.serialize();
        stream.write_all(response_str.as_bytes()).unwrap();
    }
}
