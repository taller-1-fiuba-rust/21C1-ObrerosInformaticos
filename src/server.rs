use crate::config::configuration::Configuration;
use crate::execution::Execution;
use crate::listener_thread::ListenerThread;
use crate::storage::data_storage::DataStorage;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use std::time::SystemTime;

#[allow(dead_code)]
pub struct Server {
    addr: String,
    handle: Option<JoinHandle<()>>,
    data: Arc<DataStorage>,
    config: Arc<Configuration>,
    sys_time: Arc<SystemTime>,
}

impl Server {
    pub fn new(addr: String, config: Configuration) -> Self {
        Server {
            addr,
            handle: None,
            data: Arc::new(DataStorage::new()),
            config: Arc::new(config),
            sys_time: Arc::new(SystemTime::now()),
        }
    }

    pub fn run(&mut self) {
        let mut new_addr = self.addr.clone();
        new_addr.push(':');
        let addr_and_port = new_addr + &self.config.get_port().to_string();
        let execution = Arc::new(Execution::new(
            self.data.clone(),
            self.config.clone(),
            self.sys_time.clone(),
        ));
        let handle = thread::spawn(move || {
            let listener = ListenerThread::new(addr_and_port, execution);
            listener.run();
        });
        self.handle = Some(handle);
    }

    pub fn join(&mut self) {
        if self.handle.is_none() {
            panic!("Server was joined before ran.");
        }
        let real_handle = self.handle.take().unwrap();
        real_handle.join().unwrap();
    }
}
