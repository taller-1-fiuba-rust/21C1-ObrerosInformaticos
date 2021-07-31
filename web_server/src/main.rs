mod server;
mod listener;
mod threadpool;

fn main() {
    let mut sv = server::Server::new("localhost", 8080);
    sv.run();
    sv.join();
}
