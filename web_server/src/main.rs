mod http;
mod listener;
mod server;
mod request_handler;

fn main() {
    let mut sv = server::Server::new("localhost", 8080);
    sv.run();
    sv.join();
}
