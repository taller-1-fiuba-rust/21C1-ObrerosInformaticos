use crate::config::configuration::Configuration;
use crate::execution::Execution;
use crate::listener_thread::ListenerThread;
use crate::storage::data_storage::DataStorage;
use std::net::TcpStream;
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::SystemTime;

#[allow(dead_code)]
pub struct Server {
    addr: String,
    handle: Option<JoinHandle<()>>,
    data: Arc<DataStorage>,
    config: Arc<Mutex<Configuration>>,
    sys_time: Arc<SystemTime>,
    sender: Option<Sender<()>>,
}

impl Server {
    pub fn new(config: Configuration) -> Self {
        Server {
            addr: config.get_ip().to_string(),
            handle: None,
            data: Arc::new(DataStorage::new()),
            config: Arc::new(Mutex::new(config)),
            sys_time: Arc::new(SystemTime::now()),
            sender: None,
        }
    }

    pub fn run(&mut self) {
        let addr_and_port = self.get_addr_and_port();
        let execution = Arc::new(Execution::new(
            self.data.clone(),
            self.config.clone(),
            self.sys_time.clone(),
        ));
        let ttl = self.config.lock().unwrap().get_timeout();
        let (sender, receiver) = channel();
        let handle = thread::spawn(move || {
            let listener = ListenerThread::new(addr_and_port, execution);
            listener.run(ttl, receiver);
        });
        self.sender = Some(sender);
        self.handle = Some(handle);
    }

    fn get_addr_and_port(&self) -> String {
        self.addr.clone() + ":" + &self.config.lock().unwrap().get_port().to_string()
    }

    pub fn join(&mut self) {
        if self.handle.is_none() {
            panic!("Server was joined before ran.");
        }
        let real_handle = self.handle.take().unwrap();
        real_handle.join().unwrap();
    }

    pub fn shutdown(&mut self) {
        if self.sender.is_none() {
            panic!("Server was shutdown before ran.");
        }
        let sender = self.sender.take().unwrap();
        match sender.send(()) {
            Ok(_) => {}
            Err(e) => {
                println!("{}", e);
                //panic!(e);
            }
        }
        let stream = TcpStream::connect(self.get_addr_and_port());
        drop(stream);
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        self.shutdown();
    }
}
