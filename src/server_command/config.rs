use crate::config::configuration::Configuration;
use crate::protocol::command::Command;
use crate::protocol::response::ResponseBuilder;
use std::sync::Arc;
use crate::protocol::types::ProtocolType;

pub fn run(arguments: Vec<ProtocolType>, builder: &mut ResponseBuilder, config: Arc<Configuration>) -> Result<(), &'static str>{

    if arguments[0].to_string() == "set".to_string(){
        return run_set(builder, config)
    } else if arguments[0].to_string() == "get".to_string(){
        if arguments.len() < 2{
            return Err("La cantidad de parametros es insuficiente")
        }
        return run_get(arguments, builder, config)
    }
    return Err("El argumento '{}' no existe para config.")
}

fn run_set(builder: &mut ResponseBuilder, config: Arc<Configuration>)-> Result<(), &'static str>{
    Ok(())
}

fn run_get( arguments: Vec<ProtocolType>, builder: &mut ResponseBuilder, config: Arc<Configuration>)-> Result<(), &'static str>{
    
    let verbose = "verbose".to_string();
    let port = "port".to_string();
    let id = "id".to_string();
    let logfile = "logfile".to_string();
    let dbfilename = "dbfilename".to_string();
    let timeout = "timeout".to_string();

    println!("{}", arguments[1].to_string().len());
    
    

    match arguments[1].to_string() {
        verbose => builder.add(ProtocolType::String(format!("Verbose set to: {}", config.get_verbose()))),
        port => builder.add(ProtocolType::String(format!("Port set to: {}", config.get_port()))),
        ip=> builder.add(ProtocolType::String(format!("Ip set to: {}", config.get_ip()))),
        dbfilename => builder.add(ProtocolType::String(format!("Dbfilename set to: {}", config.get_dbfilename()))),
        logfile => builder.add(ProtocolType::String(format!("Logfile set to: {}", config.get_logfile()))),
        timeout => builder.add(ProtocolType::String(format!("Timeout set to: {}", config.get_timeout()))),
        _ => builder.add(ProtocolType::String(format!("There's no configuration named: {}", arguments[2].to_string())))
    }
    
    Ok(())
}

