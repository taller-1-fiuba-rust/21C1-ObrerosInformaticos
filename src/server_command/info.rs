use crate::config::configuration::Configuration;
use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use std::sync::Arc;
use std::time::Duration;
use std::time::SystemTime;

pub fn run(
    builder: &mut ResponseBuilder,
    config: &Arc<Configuration>,
    sys_time: &Arc<SystemTime>,
) -> Result<(), &'static str> {
    let active_time: Duration = get_system_active_time(sys_time);

    let info: String = format!(
        "# Server 
redis_version:1.0 
redis_git_sha1:N/A 
redis_git_dirty:N/A
redis_build_id:N/A
redis_mode:N/A
os:Linux x86_64
arch_bits:64
gcc_version:9.3.0
process_id:N/A
process_supervised:N/A
run_id:N/A
tcp_port:{}
server_time_usec:{}
uptime_in_seconds:{}
hz:10
configured_hz:10
executable:../src/main
config_file:../config/configuration.rs
# Clients
cluster_connections:0
maxclients:32
blocked_clients:0
tracking_clients:0
clients_in_timeout_table:0
\n\r",
        config.get_port(),
        active_time.as_micros(),
        active_time.as_secs()
    );

    builder.add(ProtocolType::String(info));
    Ok(())
}

pub fn get_system_active_time(sys_time: &Arc<SystemTime>) -> Duration {
    sys_time.elapsed().unwrap()
}
