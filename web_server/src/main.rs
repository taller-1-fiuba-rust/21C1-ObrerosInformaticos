mod http;
mod listener;
mod request_handler;
mod server;

fn main() {
    let mut sv = server::Server::new("localhost", 8080);
    sv.run();
    sv.join();
}
