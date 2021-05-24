use crate::config::configuration::Configuration;
use crate::protocol::command::Command;
use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use crate::server_command::info;
use crate::storage::data_storage::DataStorage;
use std::sync::Arc;
use std::time::SystemTime;

#[allow(dead_code)]
pub struct Execution {
    data: Arc<DataStorage>,
    config: Arc<Configuration>,
    sys_time: Arc<SystemTime>,
    client_connected: u64,
}

/*
    Execution should map each command name to a function which can execute it. They don't have to necessarily be located here
*/
impl Execution {
    pub fn new(
        data: Arc<DataStorage>,
        config: Arc<Configuration>,
        sys_time: Arc<SystemTime>,
    ) -> Self {
        Execution {
            data,
            config,
            sys_time,
            client_connected: 0,
        }
    }

    pub fn run(&self, cmd: &Command, builder: &mut ResponseBuilder) -> Result<(), &'static str> {
        match &cmd.name().to_ascii_lowercase()[..] {
            "ping" => Self::pong(builder),
            "info" => info::run(builder, &self.config, &self.sys_time),
            _ => Err("Unknown command."),
        }
    }

    pub fn pong(builder: &mut ResponseBuilder) -> Result<(), &'static str> {
        builder.add(ProtocolType::String("PONG".to_string()));
        Ok(())
    }
}
