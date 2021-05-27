use std::collections::{HashMap, HashSet};
use std::io::prelude::*;
use std::net::TcpStream;
use std::sync::{RwLock, Mutex, Arc};
use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use std::sync::atomic::AtomicU32;

struct Subscriber {
    socket: Arc<Mutex<TcpStream>>,
    channel_count: u32
}

impl Subscriber {
    fn new(socket: Arc<Mutex<TcpStream>>) -> Self {
        Subscriber {
            socket,
            channel_count: 0
        }
    }
}

pub struct PublisherSubscriber {
    subscriber_ids: HashMap<i32, Subscriber>,
    subscriptions: HashMap<String, HashSet<i32>>,
    last_id: AtomicU32,
}

impl PublisherSubscriber {
    pub fn new() -> Self {
        PublisherSubscriber {
            subscriber_ids: HashMap::new(),
            subscriptions: HashMap::new(),
            last_id: AtomicU32::new(0)
        }
    }

    pub fn subscribe(&mut self, socket: Arc<Mutex<TcpStream>>, channel: &String) {
        let key = last_id;
        match self.subscriber_ids.get(&key) {
            Ok(_) => {}
            Err(_) => {
                subscriber_ids.insert(key, Subscriber::new(socket));
            }
        }
        *self.subscriptions.entry(channel.clone()).or_insert(HashSet::new()).push(key);
    }

    pub fn publish(&mut self, message: String, channel: String) -> i32 {
        let mut response = ResponseBuilder::new();
        response.add(ProtocolType::String(message));
        let response_str = response.serialize();
        let mut count = 0;

        if let Ok(streams) = self.subscriptions.get(&channel) {
            streams.retain(|subscriber| {
               match subscriber.write_all(response_str.as_bytes()) {
                   Ok(_) => {
                       count += 1;
                       true
                   },
                   Err(_) => false,
               }
            });
            self.subscriptions.insert(channel, streams);
        }
        count
    }
}