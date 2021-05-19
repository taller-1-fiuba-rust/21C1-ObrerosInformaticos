mod server;
mod listener_thread;
mod threadpool;

fn main() {
    let server = server::Server::new();
    server.run();
}
