use threadpool;

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

    fn run(&self) {
        let listener = TcpListener::bind(&self.addr).unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            pool.spawn(|| {
                &self.handle_connection(stream);
            });
        }
    }

    fn handle_connection(mut stream: TcpStream) {
        println!("Cliente received");
    }
}