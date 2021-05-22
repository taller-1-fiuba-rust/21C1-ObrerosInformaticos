use std::env;
mod configuration;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2{
        println!("Faltan parametros de inicio");
        return;
    }
    let mut configuration = configuration::Configuration::new();
    match configuration.set_config(&args[1]) {
        Err(_) => {
            // como saber si x es un tipo u otro de error
            println!("Faltan datos en el archivo de configuración.");
            return;
        }
        Ok(_) => (),
    }

    println!("{}", configuration.get_verbose());
    println!("{}", configuration.get_dbfilename());
    println!("{}", configuration.get_logfile());
    println!("{}", configuration.get_port());
    println!("{}", configuration.get_timeout());
}
