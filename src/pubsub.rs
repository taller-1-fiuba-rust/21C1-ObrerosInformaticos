use std::collections::{HashMap, HashSet};
use std::net::TcpStream;
use std::sync::{Mutex, Arc};
use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use std::sync::atomic::AtomicU32;
use std::io::Write;

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

    fn send(&mut self, msg: &String) -> Result<(), &'static str> {
        let mut locked_socket = match self.socket.lock() {
            Ok(t) => t,
            Err(e) => return Err("Failed to lock socket")
        };
        match locked_socket.write_all(msg.as_bytes()) {
            Ok(t) => Ok(t),
            Err(e) => Err(&format!("Error '{}' while writing to socket", e.to_string())[..])
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
            Some(_) => {}
            None => {
                self.subscriber_ids.insert(key, Subscriber::new(socket));
            }
        }
        self.subscriptions.entry(channel.clone()).or_insert(HashSet::new()).insert(key);
    }

    pub fn publish(&mut self, message: String, channel: String) -> i32 {
        let mut response = ResponseBuilder::new();
        response.add(ProtocolType::String(message));
        let response_str = response.serialize();
        let mut count = 0;

        if let Some(mut streams) = self.subscriptions.get(&channel) {
            streams.retain(|subscriber_id| {
                let mut subscriber = self.subscriber_ids.get(subscriber_id).unwrap();
                match subscriber.send(&response_str) {
                   Ok(_) => {
                       count += 1;
                       true
                   },
                   Err(_) => false,
                }
            });
        }
        count
    }
}