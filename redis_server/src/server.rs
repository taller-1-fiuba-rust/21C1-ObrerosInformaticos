use crate::config::configuration::Configuration;
use crate::execution::Execution;
use crate::listener_thread::ListenerThread;
use crate::logging::logger::Logger;
use crate::monitor::Monitor;
use crate::pubsub::PublisherSubscriber;
use crate::storage::data_storage::DataStorage;
use std::net::TcpStream;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use std::time::SystemTime;

// Globals
const DURATION: u64 = 600;
pub const THREADS: usize = 32;

#[allow(dead_code)]
/// A server struct
pub struct Server {
    addr: String,
    handle: Option<JoinHandle<()>>,
    handle_store_data: Option<JoinHandle<()>>,
    data: Arc<DataStorage>,
    config: Arc<Mutex<Configuration>>,
    sys_time: Arc<SystemTime>,
    logger: Arc<Logger>,
    sender: Option<Sender<()>>,
    receiver: Option<Receiver<()>>,
    is_running: bool,
}

impl Server {
    pub fn new(config: Configuration, logger: Arc<Logger>) -> Self {
        Server {
            addr: config.get_ip().to_string(),
            handle: None,
            handle_store_data: None,
            data: Arc::new(DataStorage::new()),
            config: Arc::new(Mutex::new(config)),
            sys_time: Arc::new(SystemTime::now()),
            logger,
            sender: None,
            receiver: None,
            is_running: false,
        }
    }

    /// Run the redis server
    pub fn run(&mut self) {
        let dbfile = self.config.lock().unwrap().get_dbfilename().clone();
        let result = self.data.load_data(&dbfile);
        if result.is_err() {
            println!("Error loading data from dbfile");
        };
        let addr_and_port = self.get_addr_and_port();
        let execution = Arc::new(Execution::new(
            self.data.clone(),
            self.config.clone(),
            self.sys_time.clone(),
            self.logger.clone(),
            Arc::new(PublisherSubscriber::new()),
            Monitor::new(),
        ));
        let ttl = self.config.lock().unwrap().get_timeout();
        let logger_cpy = self.logger.clone();
        let config_cpy = self.config.clone();
        let (server_sender, listener_receiver) = channel();
        let (listener_sender, server_receiver) = channel();
        let handle = thread::spawn(move || {
            let listener = ListenerThread::new(addr_and_port, execution, logger_cpy, config_cpy);
            listener.run(ttl, listener_sender, listener_receiver);
        });
        let data_storage = self.data.clone();
        let configuration = self.config.clone();
        let handle_store_data = thread::spawn(move || loop {
            loop {
                let dbfilename = configuration.lock().unwrap().get_dbfilename().clone();
                let result = data_storage.save_data(&dbfilename);
                if result.is_err() {
                    println!("Error saving data from dbfile");
                };
                let ten_mins = Duration::from_secs(DURATION);
                thread::sleep(ten_mins);
            }
        });
        self.sender = Some(server_sender);
        self.receiver = Some(server_receiver);
        self.handle = Some(handle);
        self.handle_store_data = Some(handle_store_data);
    }

    /// Returns the joined address and port
    fn get_addr_and_port(&self) -> String {
        self.addr.clone() + ":" + &self.config.lock().unwrap().get_port().to_string()
    }

    /// Checks if the server is running, if so it updates it's internal running state.
    #[allow(dead_code)]
    pub fn poll_running(&mut self) -> bool {
        if let Some(receiver) = &self.receiver {
            if receiver.try_recv().is_ok() {
                self.is_running = true;
            }
        }
        self.is_running
    }

    /// Waits for the server to finish executing
    pub fn join(&mut self) {
        if self.handle.is_none() || self.handle_store_data.is_none() {
            panic!("Server was joined before ran.");
        }
        self.handle_store_data.take().unwrap().join().unwrap();
        self.handle.take().unwrap().join().unwrap();
    }

    /// Stops listening for new connections
    pub fn shutdown(&mut self) {
        if let Some(sender) = &self.sender {
            match sender.send(()) {
                Ok(_) => {}
                Err(e) => {
                    println!("{}", e);
                    //panic!(e);
                }
            }
            let stream = TcpStream::connect(self.get_addr_and_port());
            drop(stream);
        } else {
            panic!("Server was killed before ran.");
        }
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        self.shutdown();
    }
}
