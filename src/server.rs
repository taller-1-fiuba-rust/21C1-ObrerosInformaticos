use crate::listener_thread::ListenerThread;
use std::thread;
use std::thread::JoinHandle;
use crate::storage::data_storage::DataStorage;

#[allow(dead_code)]
pub struct Server {
    addr: String,
    handle: Option<JoinHandle<()>>,
    data: DataStorage,
}

impl Server {
    pub fn new(addr: String) -> Self {
        let d = DataStorage::new();
        Server { addr, handle: None, data: d}
    }

    pub fn run(&mut self) {
        let new_addr = self.addr.clone();
        let handle = thread::spawn(move || {
            let listener = ListenerThread::new(new_addr);
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
