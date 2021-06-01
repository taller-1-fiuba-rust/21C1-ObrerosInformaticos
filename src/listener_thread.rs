use crate::execution::Execution;
use crate::protocol::command::Command;
use crate::protocol::request::Request;
use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use crate::pubsub::PublisherSubscriber;
use crate::threadpool::ThreadPool;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

pub struct ListenerThread {
    pool: ThreadPool,
    addr: String,
    execution: Arc<Execution>,
    verbosity: u8,
    pubsub: Arc<Mutex<PublisherSubscriber>>,
}

impl ListenerThread {
    pub fn new(addr: String, execution: Arc<Execution>, verbosity: u8) -> Self {
        let pool = ThreadPool::new(32);
        ListenerThread {
            pool,
            addr,
            execution,
            verbosity
            pubsub: Arc::new(Mutex::new(PublisherSubscriber::new())),
        }
    }

    pub fn run(&self, ttl: u32) {
        let listener = TcpListener::bind(&self.addr).unwrap();
        println!("REDIS server started on address '{}'...", self.addr);
        if ttl > 0 {
            listener.set_ttl(ttl).unwrap();
        }

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let exec = self.execution.clone();
            let pubsub = self.pubsub.clone();
            self.pool.spawn(move || {
                ListenerThread::handle_connection(stream, exec, pubsub);
            });
        }
    }

    fn handle_connection(
        stream: TcpStream,
        execution: Arc<Execution>,
        pubsub: Arc<Mutex<PublisherSubscriber>>,
    ) {
        let command_result = Self::parse_command(&stream);
        if let Err(e) = command_result {
            println!("{}", e);
            return;
        }
        let command = command_result.unwrap();

        Self::print_command(&command);

        Self::execute_command(&command, stream, execution, pubsub);
    }

    fn print_command(command: &Command) {
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
    }

    fn parse_command(stream: &TcpStream) -> Result<Command, String> {
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
            return Err(e);
        }

        Ok(request.build())
    }

    fn execute_command(
        command: &Command,
        stream: TcpStream,
        execution: Arc<Execution>,
        pubsub: Arc<Mutex<PublisherSubscriber>>,
    ) {
        let socket = Arc::new(Mutex::new(stream));
        let mut response = ResponseBuilder::new();

        if !execution.is_pubsub_command(&command) {
            if let Err(e) = execution.run(&command, &mut response) {
                println!("Error '{}' while running command", e);
                response.add(ProtocolType::Error(e.to_string()));
            }
        } else if let Err(e) = execution.run_pubsub(&command, &mut response, socket.clone(), pubsub)
        {
            println!("Error '{}' while running pubsub command", e);
            response.add(ProtocolType::Error(e));
        }
        Self::write_response(socket, &response);
    }

    fn write_response(stream: Arc<Mutex<TcpStream>>, response: &ResponseBuilder) {
        let mut locked_stream = match stream.lock() {
            Ok(s) => s,
            Err(_) => {
                println!("Error while writing to socket");
                return;
            }
        };
        let response_str = response.serialize();
        locked_stream.write_all(response_str.as_bytes()).unwrap();
    }
}
