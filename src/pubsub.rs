use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::net::TcpStream;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};

struct Subscriber {
    socket: Arc<Mutex<TcpStream>>,
    channels: Vec<String>,
}

impl Subscriber {
    fn new(socket: Arc<Mutex<TcpStream>>) -> Self {
        Subscriber {
            socket,
            channels: Vec::new(),
        }
    }

    fn send(&mut self, msg: &str) -> Result<(), String> {
        let mut locked_socket = match self.socket.lock() {
            Ok(t) => t,
            Err(_) => return Err("Failed to lock socket".to_string()),
        };
        match locked_socket.write_all(msg.as_bytes()) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Error '{}' while writing to socket", e.to_string())),
        }
    }
}

pub struct PublisherSubscriber {
    subscriber_ids: HashMap<u32, Subscriber>,
    subscriptions: HashMap<String, HashSet<u32>>,
    last_id: AtomicU32,
}

impl PublisherSubscriber {
    pub fn new() -> Self {
        PublisherSubscriber {
            subscriber_ids: HashMap::new(),
            subscriptions: HashMap::new(),
            last_id: AtomicU32::new(0),
        }
    }

    #[allow(dead_code)]
    ///
    /// Subscribes a socket to a specific channel, returns the number of channels the socket is subscribed to.
    ///
    pub fn subscribe(&mut self, client: Arc<Mutex<TcpStream>>, channel: &str) -> u32 {
        let client_id = self.last_id.fetch_add(1, Ordering::SeqCst);
        self.subscriber_ids
            .entry(client_id)
            .or_insert_with(|| Subscriber::new(client));
        self.subscriptions
            .entry(channel.to_string())
            .or_insert_with(HashSet::new)
            .insert(client_id);

        let sub = self.subscriber_ids.get_mut(&client_id).unwrap();
        sub.channels.push(channel.to_string());
        sub.channels.len() as u32
    }

    ///
    /// Publishes a message to a specific channel. Returns the number of subscribers which received the message.
    ///
    pub fn publish(&mut self, channel: String, message: String) -> u32 {
        let mut response = ResponseBuilder::new();
        response.add(ProtocolType::String(message));
        let response_str = response.serialize();
        let mut count = 0;

        if let Some(streams) = self.subscriptions.get_mut(&channel) {
            let subs = &mut self.subscriber_ids;
            let count_ref = &mut count;

            let mut dead_users: Vec<u32> = Vec::new();
            for subscriber_id in streams.iter() {
                let subscriber = subs.get_mut(subscriber_id).unwrap();
                let result = match subscriber.send(&response_str) {
                    Ok(_) => {
                        *count_ref += 1;
                        true
                    }
                    Err(_) => false,
                };
                if !result {
                    dead_users.push(*subscriber_id);
                }
            }

            for user in dead_users {
                self.unsubscribe(user);
            }
        }
        count
    }

    ///
    /// Unsubscribes a user from all the channels it's subscribed.
    ///
    pub fn unsubscribe(&mut self, user: u32) {
        let sub = self.subscriber_ids.get_mut(&user).unwrap();
        for channel in &sub.channels {
            self.subscriptions.get_mut(channel).unwrap().remove(&user);
        }
        self.subscriber_ids.remove(&user);
    }
}
