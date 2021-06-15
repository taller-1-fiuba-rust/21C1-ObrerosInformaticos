use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::net::TcpStream;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use crate::client::Client;

/// A pub/sub subscribers. Stores a list of channels and a socket to relay messages to.
struct Subscriber {
    channels: HashSet<String>,
    socket: Arc<Client>
}

impl Subscriber {
    fn new(client: Arc<Client>) -> Self {
        Subscriber {
            socket: client,
            channels: HashSet::new(),
        }
    }

    fn send(&self, msg: &str) -> Result<(), &'static str> {
        self.socket.send(msg)
    }
}

/// Struct which holds all the Publisher/Subscriber information. This includes
/// subscriptions, and clients.
pub struct PublisherSubscriber {
    subscriber_ids: HashMap<Arc<Client>, Subscriber>,
    subscriptions: HashMap<String, HashSet<Arc<Client>>>,
}

impl PublisherSubscriber {
    pub fn new() -> Self {
        PublisherSubscriber {
            subscriber_ids: HashMap::new(),
            subscriptions: HashMap::new()
        }
    }

    fn drop_user(&mut self, user: Arc<Client>) {
        self.subscriber_ids.remove(&user);
        user.set_pubsub_mode(false);
    }

    fn add_user(&mut self, user: Arc<Client>) {
        self.subscriber_ids
            .entry(user.clone())
            .or_insert_with(|| Subscriber::new(user.clone()));
        user.set_pubsub_mode(true);
    }

    /// Subscribes a socket to a specific channel, returns the number of channels the socket is subscribed to.
    pub fn subscribe(&mut self, client: Arc<Client>, channel: &str) -> u32 {
        self.add_user(client.clone());
        self.subscriptions
            .entry(channel.to_string())
            .or_insert_with(HashSet::new)
            .insert(client.clone());

        let sub = self.subscriber_ids.get_mut(&client).unwrap();
        sub.channels.insert(channel.to_string());
        sub.channels.len() as u32
    }

    /// Publishes a message to a specific channel. Returns the number of subscribers which received the message.
    pub fn publish(&mut self, channel: String, message: String) -> u32 {
        let response_str = Self::build_response(&channel, &message);
        let mut count = 0;

        if let Some(clients) = self.subscriptions.get(&channel) {
            let subs = &mut self.subscriber_ids;
            let count_ref = &mut count;

            let mut dead_users = Vec::new();
            for client in clients {
                let subscriber = subs.get(client).unwrap();
                let result = match subscriber.send(&response_str) {
                    Ok(_) => {
                        *count_ref += 1;
                        true
                    }
                    Err(_) => false,
                };

                if !result {
                    dead_users.push(client.clone());
                }
            }

            for user in dead_users {
                self.unsubscribe(user);
            }
        }
        count
    }

    fn build_response(channel: &String, message: &String) -> String {
        let mut response = ResponseBuilder::new();
        response.add(
            ProtocolType::Array(
                vec![
                    ProtocolType::String("message".to_string()),
                    ProtocolType::String(channel.clone()),
                    ProtocolType::String(message.clone()),
                ]
            )
        );
        response.serialize()
    }

    pub fn get_subscriptions(&self, user: Arc<Client>) -> Vec<String> {
        let sub = self.subscriber_ids.get(&user).unwrap();
        sub.channels.iter().map(|x| x.clone()).collect::<Vec<String>>()
    }

    /// Unsubscribes a user from all the channels it's subscribed.
    pub fn unsubscribe_from_channel(&mut self, user: Arc<Client>, channel: &String) -> usize {
        if let Some(sub) = self.subscriber_ids.get_mut(&user) {
            self.subscriptions.get_mut(channel).unwrap().remove(&user);
            sub.channels.remove(channel);
            let len = sub.channels.len();
            if sub.channels.is_empty() {
                self.drop_user(user);
            }
            len
        } else {
            0
        }
    }

    /// Unsubscribes a user from all the channels it's subscribed.
    fn unsubscribe(&mut self, user: Arc<Client>) {
        for channel in self.get_subscriptions(user.clone()) {
            self.unsubscribe_from_channel(user.clone(), &channel);
        }
    }
}
