use crate::config::configuration::Configuration;
use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use std::sync::Arc;

pub fn run(
    arguments: Vec<ProtocolType>,
    builder: &mut ResponseBuilder,
    config: Arc<Configuration>,
) -> Result<(), &'static str> {
    if arguments[0].to_string() == *"set" {
        // return run_set(builder, config);
    } else if arguments[0].to_string() == *"get" {
        if arguments.len() < 2 {
            return Err("La cantidad de parametros es insuficiente");
        }
        run_get(arguments, builder, config);
    }
    Err("El argumento '{}' no existe para config.")
}

// #[allow(unused_variables)]
// fn run_set(builder: &mut ResponseBuilder, config: Arc<Configuration>) -> Result<(), &'static str> {
//     Ok(())
// }

fn run_get(
    arguments: Vec<ProtocolType>,
    builder: &mut ResponseBuilder,
    config: Arc<Configuration>,
) {
    let argument: &str = &arguments[1].to_string().to_ascii_lowercase()[..];

    match argument {
        "verbose" => builder.add(ProtocolType::String(config.get_verbose().to_string())),
        "port" => builder.add(ProtocolType::String(config.get_port().to_string())),
        "ip" => builder.add(ProtocolType::String(config.get_ip().to_string())),
        "dbfilename" => builder.add(ProtocolType::String(config.get_dbfilename().to_string())),
        "logfile" => builder.add(ProtocolType::String(config.get_logfile().to_string())),
        "timeout" => builder.add(ProtocolType::String(config.get_timeout().to_string())),
        "*" => send_all_config_params(config, builder),
        _ => builder.add(ProtocolType::String(format!(
            "There's no configuration named: {}",
            arguments[1].to_string()
        ))),
    }
}
#[allow(unused_variables)]
fn send_all_config_params(config: Arc<Configuration>, builder: &mut ResponseBuilder) {
    builder.add(ProtocolType::String(format!(
        "Verbose: {}",
        config.get_verbose()
    )));
    builder.add(ProtocolType::String(format!("Port: {}", config.get_port())));
    builder.add(ProtocolType::String(format!("Ip: {}", config.get_ip())));
    builder.add(ProtocolType::String(format!(
        "Dbfilename: {}",
        config.get_dbfilename()
    )));
    builder.add(ProtocolType::String(format!(
        "Logfile: {}",
        config.get_logfile()
    )));
    builder.add(ProtocolType::String(format!(
        "Timeout: {}",
        config.get_timeout()
    )));
}
