use crate::listener_thread::ListenerThread;
use std::thread;
use std::thread::JoinHandle;

pub struct Server {
    addr: String,
    handle: Option<JoinHandle<()>>,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Server { addr, handle: None }
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