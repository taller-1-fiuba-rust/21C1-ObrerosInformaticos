use crate::config::configuration::Configuration;
use crate::logging::logger::Logger;
use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use std::sync::{Arc, Mutex};

pub fn run(
    arguments: Vec<ProtocolType>,
    builder: &mut ResponseBuilder,
    config: Arc<Mutex<Configuration>>,
    logger: Arc<Logger>,
) -> Result<(), &'static str> {
    if arguments[0].to_string().to_ascii_lowercase() == *"set" {
        return run_set(arguments, builder, config, logger);
    } else if arguments[0].to_string().to_ascii_lowercase() == *"get" {
        if arguments.len() < 2 {
            return Err("Wrong number of parameters");
        }
        run_get(arguments, builder, config);
        return Ok(());
    }
    Err("There's no configuration argument named like that")
}

fn run_set(
    arguments: Vec<ProtocolType>,
    builder: &mut ResponseBuilder,
    config: Arc<Mutex<Configuration>>,
    logger: Arc<Logger>,
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
        "timeout" => {
            let new_timeout = arguments[2].to_string();
            let new_timeout_u32 = new_timeout.parse();
            if new_timeout_u32.is_err() {
                return Err("Could not set verbosity (must be 1 or 0)");
            }
            config.lock().unwrap().set_timeout(new_timeout_u32.unwrap());
            builder.add(ProtocolType::String("Ok".to_string()));
        }
        "dbfilename" => {
            let new_dbfilename = arguments[2].to_string();
            config.lock().unwrap().set_dbfilename(new_dbfilename);
            builder.add(ProtocolType::String("Ok".to_string()));
        }
        "logfile" => {
            let new_logfile = arguments[2].to_string();
            config.lock().unwrap().set_logfile(new_logfile.clone());
            logger.change_logfile_name(new_logfile)?;
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

    let mut response = Vec::<ProtocolType>::new();
    match argument {
        "verbose" => response.push(ProtocolType::Integer(
            config.lock().unwrap().get_verbose() as i64
        )),
        "port" => response.push(ProtocolType::Integer(
            config.lock().unwrap().get_port() as i64
        )),
        "ip" => response.push(ProtocolType::String(
            config.lock().unwrap().get_ip().to_string(),
        )),
        "dbfilename" => response.push(ProtocolType::String(
            config.lock().unwrap().get_dbfilename().to_string(),
        )),
        "logfile" => response.push(ProtocolType::String(
            config.lock().unwrap().get_logfile().to_string(),
        )),
        "timeout" => response.push(ProtocolType::Integer(
            config.lock().unwrap().get_timeout() as i64
        )),
        "*" => {
            send_all_config_params(config, builder);
            return;
        }
        _ => {
            builder.add(ProtocolType::Error(format!(
                "There's no configuration named: {}",
                arguments[1].to_string()
            )));
            return;
        }
    }
    builder.add(ProtocolType::Array(response));
}
#[allow(unused_variables)]
fn send_all_config_params(config: Arc<Mutex<Configuration>>, builder: &mut ResponseBuilder) {
    let response = vec![
        ProtocolType::String(format!("Verbose: {}", config.lock().unwrap().get_verbose())),
        ProtocolType::String(format!("Port: {}", config.lock().unwrap().get_port())),
        ProtocolType::String(format!("Ip: {}", config.lock().unwrap().get_ip())),
        ProtocolType::String(format!(
            "Dbfilename: {}",
            config.lock().unwrap().get_dbfilename()
        )),
        ProtocolType::String(format!("Logfile: {}", config.lock().unwrap().get_logfile())),
        ProtocolType::String(format!("Timeout: {}", config.lock().unwrap().get_timeout())),
    ];

    builder.add(ProtocolType::Array(response));
}
