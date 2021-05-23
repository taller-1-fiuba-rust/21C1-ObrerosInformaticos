use crate::listener_thread::ListenerThread;
use crate::storage::data_storage::DataStorage;
use crate::execution::Execution;
use std::thread;
use std::thread::JoinHandle;

#[allow(dead_code)]
pub struct Server<'a> {
    addr: String,
    handle: Option<JoinHandle<()>>,
    data: DataStorage,
    execution: Execution<'a>,
}

impl<'a> Server<'a> {
    pub fn new(addr: String) -> Self {
        let d = DataStorage::new();
        let e = Execution::new(&d);
        Server {
            addr,
            handle: None,
            data: d,
            execution: e,
        }
    }

    pub fn run(&mut self) {
        let new_addr = self.addr.clone();
        let handle = thread::spawn(move || {
            let listener = ListenerThread::new(new_addr, &self.execution);
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
