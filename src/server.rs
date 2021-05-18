use std::thread;
use std::thread::JoinHandle;
use crate::listener_thread::ListenerThread;


pub struct Server {
    listener_thread: JoinHandle<()>
}

impl Server {
    pub fn new() -> Self {
        let addr = "127.0.0.1:1235".to_string();
        let handle = thread::spawn(move || {
            let listener = ListenerThread::new(addr);
            listener.run();
        });

        Server {
            listener_thread: handle
        }
    }
}

