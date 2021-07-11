use crate::config::configuration::Configuration;
use crate::execution::Execution;
use crate::logging::logger::Logger;
use crate::protocol::command::Command;

use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use crate::threadpool::ThreadPool;

use std::net::TcpListener;

use crate::client::Client;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::sync::{Arc, Mutex};

/// Struct which listens for connections and executes the given commands.
pub struct ListenerThread {
    pool: ThreadPool,
    addr: String,
    execution: Arc<Execution>,
    logger: Arc<Logger>,
    config: Arc<Mutex<Configuration>>,
}

impl ListenerThread {
    pub fn new(
        addr: String,
        execution: Arc<Execution>,
        logger: Arc<Logger>,
        config: Arc<Mutex<Configuration>>,
    ) -> Self {
        let pool = ThreadPool::new(32);
        ListenerThread {
            pool,
            addr,
            execution,
            logger,
            config,
        }
    }

    /// Listen for connections on the configured settings.
    pub fn run(&self, _ttl: u32, sx: Sender<()>, rx: Receiver<()>) {
        println!("Trying to bind on address {}", self.addr);
        let listener = match TcpListener::bind(&self.addr) {
            Ok(s) => s,
            Err(e) => {
                self.print_and_log(format!("Failed to bind to socket with error: '{}'", e));
                panic!("{}", e);
            }
        };
        self.print_and_log(format!(
            "REDIS server started on address '{}'...",
            self.addr
        ));
        sx.send(()).unwrap();

        for stream in listener.incoming() {
            match rx.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => {
                    self.print_and_log("Terminating.".to_string());
                    break;
                }
                Err(TryRecvError::Empty) => {}
            }

            let client = Arc::new(Client::new(stream.unwrap()));
            let exec = self.execution.clone();
            let logger = self.logger.clone();
            let config = self.config.clone();
            self.pool.spawn(move || {
                ListenerThread::handle_connection(client, exec, logger, config);
            });
        }
    }

    /// Handles a socket connection and executes the command extracted from it.
    fn handle_connection(
        client: Arc<Client>,
        execution: Arc<Execution>,
        logger: Arc<Logger>,
        config: Arc<Mutex<Configuration>>,
    ) {
        let commands_result = client.parse_commands();
        if let Err(e) = commands_result {
            let verbose = config.lock().unwrap().get_verbose();
            if verbose == 1 {
                println!("{}", &e);
            }
            logger.log(&e).unwrap();
            return;
        }
        let commands = commands_result.unwrap();
        for command in commands {
            Self::log_command(&command, logger.clone(), config.clone());
            Self::execute_command(
                &command,
                client.clone(),
                execution.clone(),
                logger.clone(),
                config.clone(),
            );
        }

        if !client.is_closed() {
            Self::handle_connection(client, execution, logger, config);
        }
    }

    /// Logs a given command
    fn log_command(command: &Command, logger: Arc<Logger>, config: Arc<Mutex<Configuration>>) {
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
        let verbose = config.lock().unwrap().get_verbose();
        if verbose == 1 {
            println!("{}", &msg);
        }
        logger.log(&msg).unwrap();
    }

    /// Executed a given command.
    fn execute_command(
        command: &Command,
        client: Arc<Client>,
        execution: Arc<Execution>,
        logger: Arc<Logger>,
        config: Arc<Mutex<Configuration>>,
    ) {
        let mut response = ResponseBuilder::new();

        if let Err(e) = execution.run(&command, &mut response, client.clone()) {
            logger.log("Error").unwrap();
            response.add(ProtocolType::Error(e.to_string()));
        }
        Self::write_response(client, &response, logger, config);
    }

    /// Write a response from a response builder to the desired socket.
    fn write_response(
        client: Arc<Client>,
        response: &ResponseBuilder,
        logger: Arc<Logger>,
        config: Arc<Mutex<Configuration>>,
    ) {
        let verbose = config.lock().unwrap().get_verbose();
        if verbose == 1 {
            println!("{}", &response.to_string());
        }
        logger.log(&response.to_string()).unwrap();
        client.send(&response.serialize()).unwrap();
    }

    fn print_and_log(&self, msg: String) {
        println!("{}", msg);
        self.logger.log(&msg).unwrap();
    }
}
