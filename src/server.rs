use listener_thread;


pub struct Server {
    listener_thread: JoinHandle<_>
}

impl Server {
    pub fn new() -> Self {
        let addr = "127.0.0.1:1235";
        let handle = thread::spawn(move || {
            let listener = ListenerThread::new(addr);
            listener.run();
        });

        Server {
            listener_thread: handle
        }
    }
}

