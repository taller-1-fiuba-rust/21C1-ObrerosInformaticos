use std::net::TcpStream;
use std::net::TcpListener;
use crate::threadpool::ThreadPool;

pub struct ListenerThread {
    pool: ThreadPool,
    addr: String,
}

impl ListenerThread {
    pub fn new(addr: String) -> Self {
        let pool = ThreadPool::new(32);

        ListenerThread {
            pool,
            addr
        }
    }

    pub fn run(&self) {
        let listener = TcpListener::bind(&self.addr).unwrap();
        println!("Listening...");
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            self.pool.spawn(|| {
                ListenerThread::handle_connection(stream);
            });
        }
    }

    fn handle_connection(mut stream: TcpStream) {
        println!("Cliente received");
    }
}