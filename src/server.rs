use crate::config::configuration::Configuration;
use crate::execution::Execution;
use crate::listener_thread::ListenerThread;
use crate::storage::data_storage::DataStorage;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use std::time::SystemTime;

#[allow(dead_code)]
pub struct Server {
    addr: String,
    handle: Option<JoinHandle<()>>,
    handle_store_data: Option<JoinHandle<()>>,
    data: Arc<DataStorage>,
    config: Arc<Mutex<Configuration>>,
    sys_time: Arc<SystemTime>,
}

impl Server {
    pub fn new(config: Configuration) -> Self {
        Server {
            addr: config.get_ip().to_string(),
            handle: None,
            handle_store_data: None,
            data: Arc::new(DataStorage::new()),
            config: Arc::new(Mutex::new(config)),
            sys_time: Arc::new(SystemTime::now()),
        }
    }

    pub fn run(&mut self) {
        let dbfile = self.config.lock().unwrap().get_dbfilename().clone();
        let result = self.data.load_data(&dbfile);
        if result.is_err() {
            println!("Error loading data from dbfile");
        };
        let mut new_addr = self.addr.clone();
        new_addr.push(':');
        let addr_and_port = new_addr + &self.config.lock().unwrap().get_port().to_string();
        let execution = Arc::new(Execution::new(
            self.data.clone(),
            self.config.clone(),
            self.sys_time.clone(),
        ));
        let ttl = self.config.lock().unwrap().get_timeout();

        let handle = thread::spawn(move || {
            let listener = ListenerThread::new(addr_and_port, execution);
            listener.run(ttl);
        });

        let data_storage = self.data.clone();
        let handle_store_data = thread::spawn(move || loop {
            loop {
                let result = data_storage.save_data(&dbfile);
                if result.is_err() {
                    println!("Error saving data from dbfile");
                };
                let ten_mins = Duration::from_secs(600);
                thread::sleep(ten_mins);
            }
        });

        self.handle = Some(handle);
        self.handle_store_data = Some(handle_store_data);
    }

    pub fn join(&mut self) {
        if self.handle.is_none() || self.handle_store_data.is_none() {
            panic!("Server was joined before ran.");
        }
        let real_handle = self.handle.take().unwrap();
        real_handle.join().unwrap();

        let real_handle_store_data = self.handle_store_data.take().unwrap();
        real_handle_store_data.join().unwrap();
    }
}
