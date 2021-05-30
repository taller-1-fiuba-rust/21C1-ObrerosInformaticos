use crate::config::configuration::Configuration;
use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use std::sync::{Arc, Mutex};

pub fn run(
    arguments: Vec<ProtocolType>,
    builder: &mut ResponseBuilder,
    config: Arc<Mutex<Configuration>>,
) -> Result<(), &'static str> {
    if arguments[0].to_string() == *"set" {
        return run_set(arguments, builder, config);
    } else if arguments[0].to_string() == *"get" {
        if arguments.len() < 2 {
            return Err("Wrong number of parameters");
        }
        run_get(arguments, builder, config);
        return Ok(());
    }
    Err("There's no configuration argument named like that.")
}

fn run_set(
    arguments: Vec<ProtocolType>,
    builder: &mut ResponseBuilder,
    config: Arc<Mutex<Configuration>>,
) -> Result<(), &'static str> {
    let argument: &str = &arguments[1].to_string().to_ascii_lowercase()[..];

    match argument {
        "verbose" => {
            let new_verb: String = arguments[2].to_string();
            let new_verb_u8 = new_verb.parse();
            if new_verb_u8.is_err() {
                return Err("Could not set verbosity (must be 1 or 0)");
            }
            let config_res = config.lock().unwrap().set_verbose(new_verb_u8.unwrap());
            if let Err(er) = config_res {
                return Err(er);
            }
            builder.add(ProtocolType::String("Ok".to_string()));
        }
        _ => builder.add(ProtocolType::String(format!(
            "There's no configuration named: {}",
            arguments[1].to_string()
        ))),
    }
    Ok(())
}

fn run_get(
    arguments: Vec<ProtocolType>,
    builder: &mut ResponseBuilder,
    config: Arc<Mutex<Configuration>>,
) {
    let argument: &str = &arguments[1].to_string().to_ascii_lowercase()[..];

    match argument {
        "verbose" => builder.add(ProtocolType::String(
            config.lock().unwrap().get_verbose().to_string(),
        )),
        "port" => builder.add(ProtocolType::String(
            config.lock().unwrap().get_port().to_string(),
        )),
        "ip" => builder.add(ProtocolType::String(
            config.lock().unwrap().get_ip().to_string(),
        )),
        "dbfilename" => builder.add(ProtocolType::String(
            config.lock().unwrap().get_dbfilename().to_string(),
        )),
        "logfile" => builder.add(ProtocolType::String(
            config.lock().unwrap().get_logfile().to_string(),
        )),
        "timeout" => builder.add(ProtocolType::String(
            config.lock().unwrap().get_timeout().to_string(),
        )),
        "*" => send_all_config_params(config, builder),
        _ => builder.add(ProtocolType::String(format!(
            "There's no configuration named: {}",
            arguments[1].to_string()
        ))),
    }
}
#[allow(unused_variables)]
fn send_all_config_params(config: Arc<Mutex<Configuration>>, builder: &mut ResponseBuilder) {
    builder.add(ProtocolType::String(format!(
        "Verbose: {}",
        config.lock().unwrap().get_verbose()
    )));
    builder.add(ProtocolType::String(format!(
        "Port: {}",
        config.lock().unwrap().get_port()
    )));
    builder.add(ProtocolType::String(format!(
        "Ip: {}",
        config.lock().unwrap().get_ip()
    )));
    builder.add(ProtocolType::String(format!(
        "Dbfilename: {}",
        config.lock().unwrap().get_dbfilename()
    )));
    builder.add(ProtocolType::String(format!(
        "Logfile: {}",
        config.lock().unwrap().get_logfile()
    )));
    builder.add(ProtocolType::String(format!(
        "Timeout: {}",
        config.lock().unwrap().get_timeout()
    )));
}
