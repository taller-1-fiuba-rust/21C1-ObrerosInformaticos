use crate::config::configuration::Configuration;
use crate::key_command::{copy, rename};
use crate::key_command::{expire, persist};
use crate::protocol::command::Command;
use crate::protocol::response::ResponseBuilder;
use crate::pubsub::PublisherSubscriber;
use crate::server_command::config;
use crate::server_command::info;
use crate::server_command::ping;
use crate::server_command::pubsub;
use crate::storage::data_storage::DataStorage;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

#[allow(dead_code)]
pub struct Execution {
    data: Arc<DataStorage>,
    config: Arc<Mutex<Configuration>>,
    sys_time: Arc<SystemTime>,
    client_connected: u64,
}

/*
    Execution should map each command name to a function which can execute it. They don't have to necessarily be located here
*/
impl Execution {
    pub fn new(
        data: Arc<DataStorage>,
        config: Arc<Mutex<Configuration>>,
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
            "ping" => ping::run(builder),
            "info" => info::run(builder, &self.config, &self.sys_time),
            "expire" => expire::run(builder, cmd, &self.data),
            "copy" => copy::run(self.data.clone(), cmd.arguments(), builder),
            "rename" => rename::run(self.data.clone(), cmd.arguments(), builder),
            "persist" => persist::run(self.data.clone(), cmd.arguments(), builder),
            "config" => config::run(cmd.arguments(), builder, self.config.clone()),
            _ => Err("Unknown command."),
        }
    }

    #[allow(unused_variables)]
    pub fn run_pubsub(
        &self,
        cmd: &Command,
        response: &mut ResponseBuilder,
        socket: Arc<Mutex<TcpStream>>,
        pubsub: Arc<Mutex<PublisherSubscriber>>,
    ) -> Result<(), String> {
        match &cmd.name().to_ascii_lowercase()[..] {
            "subscribe" => pubsub::subscribe::run(pubsub, socket, response, cmd.arguments()),
            "publish" => pubsub::publish::run(pubsub, response, cmd.arguments()),
            _ => Err("Unknown command.".to_string()),
        }
    }

    pub fn is_pubsub_command(&self, cmd: &Command) -> bool {
        matches!(
            &cmd.name().to_ascii_lowercase()[..],
            "subscribe" | "publish"
        )
    }
}
