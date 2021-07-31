use crate::config::configuration::Configuration;
use redis_protocol::response::ResponseBuilder;
use redis_protocol::types::ProtocolType;
use crate::server::THREADS;
use std::env;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::time::SystemTime;

///Funcion para la ejecucion del comando INFO.
///Imprime informacion del servidor y los clientes conectados.
pub fn run(
    builder: &mut ResponseBuilder,
    config: &Arc<Mutex<Configuration>>,
    sys_time: &Arc<SystemTime>,
) -> Result<(), &'static str> {
    let active_time: Duration = get_system_active_time(sys_time);
    let cfg_lock = config.lock().unwrap();
    let info: String = format!(
        "# Server 
redis_version:1.0
redis_mode:N/A
os:{}
arch_bits:64
process_id:N/A
process_supervised:N/A
run_id:N/A
tcp_port:{}
server_time_usec:{}
uptime_in_seconds:{}
executable:{}
config_file:{}
# Clients
maxclients:{}
blocked_clients:0
tracking_clients:0
clients_in_timeout_table:0
\n\r",
        env::consts::OS,
        cfg_lock.get_port(),
        active_time.as_micros(),
        active_time.as_secs(),
        env::current_exe().unwrap().to_str().unwrap(),
        match cfg_lock.get_configfile() {
            Some(v) => v,
            None => "None".to_string(),
        },
        THREADS,
    );

    builder.add(ProtocolType::String(info));
    Ok(())
}

///Obtiene tiempo total en el cual el servidor se encontro activo.
pub fn get_system_active_time(sys_time: &Arc<SystemTime>) -> Duration {
    sys_time.elapsed().unwrap()
}
