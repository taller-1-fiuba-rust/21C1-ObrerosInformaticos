use proyecto_taller_1::config::configuration::Configuration;
use proyecto_taller_1::logging::logger::Logger;
use proyecto_taller_1::server::Server;

static SERVER_PORT u16 = 10010;

fn main() {
    let mut config = Configuration::new();
    let logger: Arc<Logger> = Arc::new(Logger::new(config.get_logfile()).unwrap());
    config.set_port(SERVER_PORT);

    let mut sv = Server::new(config, logger);
    sv.run();

    //VER ESTRUCTURA DE MAXI.
    let request = ResquestSTRUCT;
    for request in RequestSTRUCT {
        match request {
            Ok(request) => {
                // LLAMAR AL SERVER DE REDIS CON EL REQUEST
                // OBTENER LA RESPUESTA
                // ENVIARLE A MAXI LA RESPUESTA
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    
    sv.join();
}