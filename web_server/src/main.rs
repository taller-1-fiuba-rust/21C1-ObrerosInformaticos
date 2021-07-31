mod server;
mod listener;
mod http;

fn main() {
    let mut sv = server::Server::new("localhost", 8080);
    sv.run();
    sv.join();
}
