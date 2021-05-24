use crate::listener_thread::ListenerThread;
use crate::storage::data_storage::DataStorage;
use crate::execution::Execution;
use std::thread;
use std::thread::JoinHandle;
use std::sync::Arc;

#[allow(dead_code)]
pub struct Server {
    addr: String,
    handle: Option<JoinHandle<()>>,
    data: Arc<DataStorage>,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Server {
            addr,
            handle: None,
            data: Arc::new(DataStorage::new()),
        }
    }

    pub fn run(&mut self) {
        let new_addr = self.addr.clone();
        let execution = Arc::new(Execution::new(self.data.clone()));
        let handle = thread::spawn(move || {
            let listener = ListenerThread::new(new_addr, execution);
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
