use std::thread;
use crate::listener_thread::ListenerThread;


pub struct Server {
}

impl Server {
    pub fn new() -> Self {
        Server {
        }
    }

    pub fn run(&self) {
        let addr = "127.0.0.1:1234".to_string();
        let handle = thread::spawn(move || {
            let listener = ListenerThread::new(addr);
            listener.run();
        });
        handle.join().unwrap();
    }
}

