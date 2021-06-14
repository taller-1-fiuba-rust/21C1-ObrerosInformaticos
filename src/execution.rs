use crate::config::configuration::Configuration;
use crate::key_command::{
    copy, del, exists, expire, expireat, key_type, keys, persist, rename, sort, touch, ttl,
};
use crate::logging::logger::Logger;
use crate::protocol::command::Command;
use crate::protocol::response::ResponseBuilder;
use crate::pubsub::PublisherSubscriber;
use crate::server_command::{config, info, ping, pubsub};
use crate::storage::data_storage::DataStorage;
use crate::string_command::{append, decrby, get, getdel, getset, mset, set, strlen, mget};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

#[allow(dead_code)]
pub struct Execution {
    data: Arc<DataStorage>,
    config: Arc<Mutex<Configuration>>,
    sys_time: Arc<SystemTime>,
    client_connected: u64,
    logger: Arc<Logger>,
}

impl Execution {
    pub fn new(
        data: Arc<DataStorage>,
        config: Arc<Mutex<Configuration>>,
        sys_time: Arc<SystemTime>,
        logger: Arc<Logger>,
    ) -> Self {
        Execution {
            data,
            config,
            sys_time,
            client_connected: 0,
            logger,
        }
    }

    /// Matches a command with it's executing function and runs it.
    pub fn run(&self, cmd: &Command, builder: &mut ResponseBuilder) -> Result<(), &'static str> {
        match &cmd.name().to_ascii_lowercase()[..] {
            "ping" => ping::run(builder),
            "info" => info::run(builder, &self.config, &self.sys_time),
            "expire" => expire::run(builder, cmd, &self.data),
            "expireat" => expireat::run(builder, cmd.arguments(), &self.data),
            "copy" => copy::run(self.data.clone(), cmd.arguments(), builder),
            "keys" => keys::run(self.data.clone(), cmd.arguments(), builder),
            "rename" => rename::run(self.data.clone(), cmd.arguments(), builder),
            "persist" => persist::run(self.data.clone(), cmd.arguments(), builder),
            "config" => config::run(cmd.arguments(), builder, self.config.clone()),
            "type" => key_type::run(cmd.arguments(), builder, &self.data),
            "del" => del::run(builder, cmd.arguments(), &self.data),
            "sort" => sort::run(builder, cmd.arguments(), &self.data),
            "exists" => exists::run(builder, cmd.arguments(), &self.data),
            "ttl" => ttl::run(builder, cmd.arguments(), &self.data),
            "touch" => touch::run(builder, cmd.arguments(), &self.data),
            "mset" => mset::run(self.data.clone(), cmd.arguments(), builder),
            "set" => set::run(self.data.clone(), cmd.arguments(), builder),
            "strlen" => strlen::run(self.data.clone(), cmd.arguments(), builder),
            "getset" => getset::run(builder, cmd.arguments(), &self.data),
            "decrby" => decrby::run(self.data.clone(), cmd.arguments(), builder),
            "append" => append::run(cmd.arguments(), builder, self.data.clone()),
            "getdel" => getdel::run(cmd.arguments(), builder, self.data.clone()),
            "get" => get::run(cmd.arguments(), builder, self.data.clone()),
            "mget" => mget::run(cmd.arguments(), builder, self.data.clone()),
            _ => Err("Unknown command."),
        }
    }

    /// Executes pub/sub commands. This distinction is required because of mode changes related to pub/sub commands.
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

    /// Returns a bool representing if the command is a pub/sub topic command or not.
    pub fn is_pubsub_command(&self, cmd: &Command) -> bool {
        matches!(
            &cmd.name().to_ascii_lowercase()[..],
            "subscribe" | "publish"
        )
    }
}
