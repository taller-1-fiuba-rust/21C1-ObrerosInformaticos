mod listener_thread;
mod server;
mod threadpool;
mod request;
mod command;
mod types;

fn main() {
    let addr = "127.0.0.1:1234".to_string();
    let mut server = server::Server::new(addr);
    server.run();
    server.join();
}
