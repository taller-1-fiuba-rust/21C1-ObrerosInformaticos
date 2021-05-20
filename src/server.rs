use std::thread;
use crate::listener_thread::ListenerThread;
use std::thread::JoinHandle;


pub struct Server {
    addr: String,
    handle: Option<JoinHandle<()>>
}

impl Server {
    pub fn new(addr: String) -> Self {
        Server {
            addr,
            handle: None
        }
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

#[cfg(test)]
mod tests {

    use super::*;
    use std::net::TcpStream;

    #[test]
    fn test_run() {
        let addr = "127.0.0.1:34254".to_string();
        let mut server = Server::new(addr.clone());
        server.run();

        let result = TcpStream::connect(&addr);
        assert!(result.is_ok());
    }
}

