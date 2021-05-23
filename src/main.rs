use std::env;
mod configuration;

use crate::configuration::configuration::Configuration;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Faltan parametros de inicio");
        return;
    }

    let mut configuration = Configuration::new();

    if let Err(msj) = configuration.set_config(&args[1]) {
        println!("{}", msj);
        return;
    }
}
