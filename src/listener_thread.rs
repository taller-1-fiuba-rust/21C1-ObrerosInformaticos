use crate::execution::Execution;
use crate::logging::logger::Logger;
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

/// Struct which listens for connections and executes the given commands.
pub struct ListenerThread {
    pool: ThreadPool,
    addr: String,
    execution: Arc<Execution>,
    logger: Arc<Mutex<Logger>>,
    pubsub: Arc<Mutex<PublisherSubscriber>>,
}

impl ListenerThread {
    pub fn new(addr: String, execution: Arc<Execution>, logger: Arc<Mutex<Logger>>) -> Self {
        let pool = ThreadPool::new(32);
        ListenerThread {
            pool,
            addr,
            execution,
            logger,
            pubsub: Arc::new(Mutex::new(PublisherSubscriber::new())),
        }
    }

    /// Listen for connections on the configured settings.
    pub fn run(&self, _ttl: u32) {
        let listener = TcpListener::bind(&self.addr).unwrap();
        match self.logger.lock() {
            Ok(lock) => {
                let msg = format!("REDIS server started on address '{}'...", self.addr);
                lock.log(&msg).unwrap();
            }
            Err(_) => println!("Error while getting the logger lock"),
        };

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let exec = self.execution.clone();
            let pubsub = self.pubsub.clone();
            let logger = self.logger.clone();
            self.pool.spawn(move || {
                ListenerThread::handle_connection(stream, exec, pubsub, logger);
            });
        }
    }

    /// Handles a socket connection and executes the command extracted from it.
    fn handle_connection(
        stream: TcpStream,
        execution: Arc<Execution>,
        pubsub: Arc<Mutex<PublisherSubscriber>>,
        logger: Arc<Mutex<Logger>>,
    ) {
        let command_result = Self::parse_command(&stream);
        if let Err(e) = command_result {
            match logger.lock() {
                Ok(lock) => lock.log(&e).unwrap(),
                Err(_) => println!("Error while getting the logger lock"),
            };
            return;
        }
        let command = command_result.unwrap();

        Self::print_command(&command, logger.clone());

        Self::execute_command(&command, stream, execution, pubsub, logger);
    }

    /// Prints a given command
    fn print_command(command: &Command, logger: Arc<Mutex<Logger>>) {
        match logger.lock() {
            Ok(lock) => {
                let msg = format!(
                    "Received command '{} {}'",
                    command.name(),
                    command
                        .arguments()
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(" ")
                );
                lock.log(&msg).unwrap();
            }
            Err(_) => println!("Error while getting the logger lock"),
        };
    }

    /// Parses a command from a socket connection
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

    /// Executed a given command.
    fn execute_command(
        command: &Command,
        stream: TcpStream,
        execution: Arc<Execution>,
        pubsub: Arc<Mutex<PublisherSubscriber>>,
        logger: Arc<Mutex<Logger>>,
    ) {
        let socket = Arc::new(Mutex::new(stream));
        let mut response = ResponseBuilder::new();

        if !execution.is_pubsub_command(&command) {
            if let Err(e) = execution.run(&command, &mut response) {
                match logger.lock() {
                    Ok(lock) => lock.log("Error").unwrap(),
                    Err(_) => println!("Error while getting the logger lock"),
                };
                response.add(ProtocolType::Error(e.to_string()));
            }
        } else if let Err(e) = execution.run_pubsub(&command, &mut response, socket.clone(), pubsub)
        {
            match logger.lock() {
                Ok(lock) => lock.log("Error").unwrap(),
                Err(_) => println!("Error while getting the logger lock"),
            };
            response.add(ProtocolType::Error(e));
        }
        Self::write_response(socket, &response, logger);
    }

    /// Write a response from a response builder to the desired socket.
    fn write_response(
        stream: Arc<Mutex<TcpStream>>,
        response: &ResponseBuilder,
        logger: Arc<Mutex<Logger>>,
    ) {
        let mut locked_stream = match stream.lock() {
            Ok(s) => s,
            Err(_) => {
                println!("Error while writing to socket");
                return;
            }
        };
        let response_str = response.serialize();

        match logger.lock() {
            Ok(lock) => lock.log(&response_str).unwrap(),
            Err(_) => println!("Error while getting the logger lock"),
        };

        locked_stream.write_all(response_str.as_bytes()).unwrap();
    }
}
