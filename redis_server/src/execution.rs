use crate::client::Client;
use crate::config::configuration::Configuration;
use crate::key_command::{
    copy, del, exists, expire, expireat, key_type, keys, persist, rename, sort, touch, ttl,
};
use crate::lists_command::{
    lindex, llen, lpop, lpush, lpushx, lrange, lrem, lset, rpop, rpush, rpushx,
};
use crate::logging::logger::Logger;
use crate::monitor::Monitor;
use crate::pubsub::PublisherSubscriber;
use crate::pubsub_command::{publish, pubsub, punsubscribe, subscribe, unsubscribe};
use crate::server_command::{config, dbsize, flushdb, info, monitor, ping, quit};
use crate::set_command::{sadd, scard, sismember, smembers, srem};
use crate::storage::data_storage::DataStorage;
use crate::string_command::{append, decrby, get, getdel, getset, incrby, mget, mset, set, strlen};
use redis_protocol::command::Command;
use redis_protocol::response::ResponseBuilder;
use redis_protocol::types::ProtocolType;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

#[allow(dead_code)]
/// Struct which holds an execution context for the server
pub struct Execution {
    data: Arc<DataStorage>,
    config: Arc<Mutex<Configuration>>,
    sys_time: Arc<SystemTime>,
    client_connected: u64,
    logger: Arc<Logger>,
    pubsub: Arc<PublisherSubscriber>,
    monitor: Monitor,
}

impl Execution {
    pub fn new(
        data: Arc<DataStorage>,
        config: Arc<Mutex<Configuration>>,
        sys_time: Arc<SystemTime>,
        logger: Arc<Logger>,
        pubsub: Arc<PublisherSubscriber>,
        monitor: Monitor,
    ) -> Self {
        Execution {
            data,
            config,
            sys_time,
            client_connected: 0,
            logger,
            pubsub,
            monitor,
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

        if self.monitor.is_active() {
            let msg = get_message(cmd);
            self.monitor.send(&msg.serialize())?;
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
            "config" => config::run(
                cmd.arguments(),
                builder,
                self.config.clone(),
                self.logger.clone(),
            ),
            "type" => key_type::run(cmd.arguments(), builder, &self.data),
            "del" => del::run(builder, cmd.arguments(), &self.data),
            "sort" => sort::run(builder, cmd.arguments(), &self.data),
            "exists" => exists::run(builder, cmd.arguments(), &self.data),
            "ttl" => ttl::run(builder, cmd.arguments(), &self.data),
            "touch" => touch::run(builder, cmd.arguments(), &self.data, self.logger.clone()),
            "mset" => mset::run(self.data.clone(), cmd.arguments(), builder),
            "set" => set::run(self.data.clone(), cmd.arguments(), builder),
            "strlen" => strlen::run(self.data.clone(), cmd.arguments(), builder),
            "getset" => getset::run(builder, cmd.arguments(), &self.data),
            "decrby" => decrby::run(self.data.clone(), cmd.arguments(), builder),
            "incrby" => incrby::run(self.data.clone(), cmd.arguments(), builder),
            "append" => append::run(cmd.arguments(), builder, self.data.clone()),
            "getdel" => getdel::run(cmd.arguments(), builder, self.data.clone()),
            "get" => get::run(cmd.arguments(), builder, self.data.clone()),
            "mget" => mget::run(cmd.arguments(), builder, self.data.clone()),
            "unsubscribe" => {
                unsubscribe::run(self.pubsub.clone(), client, builder, cmd.arguments())
            }
            "subscribe" => subscribe::run(self.pubsub.clone(), client, builder, cmd.arguments()),
            "publish" => publish::run(self.pubsub.clone(), builder, cmd.arguments()),
            "punsubscribe" => punsubscribe::run(self.pubsub.clone(), client, builder),
            "pubsub" => pubsub::run(self.pubsub.clone(), builder, cmd.arguments()),
            "flushdb" => flushdb::run(builder, self.data.clone()),
            "dbsize" => dbsize::run(builder, self.data.clone()),
            "lpushx" => lpushx::run(builder, cmd.arguments(), self.data.clone()),
            "lset" => lset::run(builder, cmd.arguments(), self.data.clone()),
            "rpushx" => rpushx::run(builder, cmd.arguments(), self.data.clone()),
            "rpush" => rpush::run(builder, cmd.arguments(), self.data.clone()),
            "rpop" => rpop::run(builder, cmd.arguments(), self.data.clone()),
            "lindex" => lindex::run(cmd.arguments(), builder, self.data.clone()),
            "lpush" => lpush::run(builder, cmd.arguments(), self.data.clone()),
            "llen" => llen::run(cmd.arguments(), builder, self.data.clone()),
            "lpop" => lpop::run(cmd.arguments(), builder, self.data.clone()),
            "lrem" => lrem::run(builder, cmd.arguments(), self.data.clone()),
            "sismember" => sismember::run(builder, cmd.arguments(), self.data.clone()),
            "smembers" => smembers::run(builder, cmd.arguments(), self.data.clone()),
            "srem" => srem::run(builder, cmd.arguments(), self.data.clone()),
            "scard" => scard::run(builder, cmd.arguments(), self.data.clone()),
            "sadd" => sadd::run(builder, cmd.arguments(), self.data.clone()),
            "lrange" => lrange::run(builder, cmd.arguments(), self.data.clone()),
            "monitor" => monitor::run(&self.monitor, client, builder),
            "quit" => quit::run(&self.monitor, client, builder),
            _ => Err("Unknown command."),
        }
    }
}

fn get_message(cmd: &Command) -> ResponseBuilder {
    let mut command = cmd.name();
    if command == "COMMAND" {
        command = "NEW CONNECTION".to_string();
    }
    let mut arguments: Vec<String> = cmd
        .arguments()
        .into_iter()
        .map(|x| x.string())
        .collect::<Result<_, _>>()
        .unwrap();
    arguments.insert(0, command);
    let mut msg = ResponseBuilder::new();
    msg.add(ProtocolType::String(arguments.join(" ")));
    msg
}
