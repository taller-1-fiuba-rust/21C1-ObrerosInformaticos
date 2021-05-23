mod execution;
mod listener_thread;
mod protocol;
mod server;
mod storage;
mod threadpool;
mod server_command;

fn main() {
    let addr = "127.0.0.1:6379".to_string();
    let mut server = server::Server::new(addr);
    server.run();
    server.join();
}
