use crate::config::configuration::Configuration;
use crate::key_command::{
    copy, del, exists, expire, expireat, key_type, keys, persist, rename, sort, touch, ttl,
};
use crate::logging::logger::Logger;
use crate::protocol::command::Command;
use crate::protocol::response::ResponseBuilder;
use crate::pubsub::PublisherSubscriber;
use crate::pubsub_command::{publish, pubsub, punsubscribe, subscribe, unsubscribe};
use crate::server_command::{config, dbsize, info, ping};
use crate::storage::data_storage::DataStorage;
use crate::string_command::{append, decrby, get, getdel, getset, mset, set, strlen};

use crate::client::Client;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

#[allow(dead_code)]
pub struct Execution {
    data: Arc<DataStorage>,
    config: Arc<Mutex<Configuration>>,
    sys_time: Arc<SystemTime>,
    client_connected: u64,
    logger: Arc<Logger>,
    pubsub: Arc<Mutex<PublisherSubscriber>>,
}

impl Execution {
    pub fn new(
        data: Arc<DataStorage>,
        config: Arc<Mutex<Configuration>>,
        sys_time: Arc<SystemTime>,
        logger: Arc<Logger>,
        pubsub: Arc<Mutex<PublisherSubscriber>>,
    ) -> Self {
        Execution {
            data,
            config,
            sys_time,
            client_connected: 0,
            logger,
            pubsub,
        }
    }

    /// Matches a command with it's executing function and runs it.
    pub fn run(
        &self,
        cmd: &Command,
        builder: &mut ResponseBuilder,
        client: Arc<Client>,
    ) -> Result<(), &'static str> {
        if client.in_pubsub_mode()
            && !matches!(
                &cmd.name().to_uppercase()[..],
                "SUBSCRIBE" | "PSUBSCRIBE" | "UNSUBSCRIBE" | "PUNSUBSCRIBE" | "PING" | "QUIT"
            )
        {
            return Err("A client in pub/sub mode can only use SUBSCRIBE, PSUBSCRIBE, UNSUBSCRIBE, PUNSUBSCRIBE, PING and QUIT");
        }

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
            "unsubscribe" => {
                unsubscribe::run(self.pubsub.clone(), client, builder, cmd.arguments())
            }
            "subscribe" => subscribe::run(self.pubsub.clone(), client, builder, cmd.arguments()),
            "publish" => publish::run(self.pubsub.clone(), builder, cmd.arguments()),
            "punsubscribe" => punsubscribe::run(self.pubsub.clone(), client, builder),
            "pubsub" => pubsub::run(self.pubsub.clone(), builder, cmd.arguments()),
            "dbsize" => dbsize::run(builder, self.data.clone()),
            _ => Err("Unknown command."),
        }
    }
}
