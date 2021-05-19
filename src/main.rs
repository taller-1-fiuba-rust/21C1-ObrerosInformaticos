use std::env;
mod configuration;

fn main() {
    let args: Vec<String> = env::args().collect();
    let configuration = configuration::Configuration::new(&args[1]);
}