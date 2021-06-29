use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use std::collections::{HashMap, HashSet};

use crate::client::Client;
use std::sync::{Arc, RwLock};

/// A pub/sub subscribers. Stores a list of channels and a socket to relay messages to.
struct Subscriber {
    channels: HashSet<String>,
    socket: Arc<Client>,
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
    users: RwLock<HashMap<Arc<Client>, Subscriber>>,
    subscriptions: RwLock<HashMap<String, HashSet<Arc<Client>>>>,
}

impl PublisherSubscriber {
    pub fn new() -> Self {
        PublisherSubscriber {
            users: RwLock::new(HashMap::new()),
            subscriptions: RwLock::new(HashMap::new()),
        }
    }

    fn drop_user(&self, user: Arc<Client>) -> Result<(), &'static str> {
        let mut users = self.users.write().ok().ok_or("Failed to lock")?;
        users.remove(&user);
        user.set_pubsub_mode(false);
        Ok(())
    }

    fn add_user(&self, user: Arc<Client>) -> Result<(), &'static str> {
        let mut users = self.users.write().ok().ok_or("Failed to lock")?;
        users
            .entry(user.clone())
            .or_insert_with(|| Subscriber::new(user.clone()));
        user.set_pubsub_mode(true);
        Ok(())
    }

    /// Subscribes a socket to a specific channel, returns the number of channels the socket is subscribed to.
    pub fn subscribe(&self, client: Arc<Client>, channel: &str) -> Result<u32, &'static str> {
        self.add_user(client.clone())?;
        let mut subscriptions = self.subscriptions.write().ok().ok_or("Failed to lock")?;
        subscriptions
            .entry(channel.to_string())
            .or_insert_with(HashSet::new)
            .insert(client.clone());

        let mut users = self.users.write().ok().ok_or("Failed to lock")?;
        let sub = users.get_mut(&client).unwrap();
        sub.channels.insert(channel.to_string());
        Ok(sub.channels.len() as u32)
    }

    /// Publishes a message to a specific channel. Returns the number of subscribers which received the message.
    pub fn publish(&self, channel: String, message: String) -> Result<u32, &'static str> {
        let response_str = Self::build_response(&channel, &message);
        let mut count = 0;

        let subscriptions = self.subscriptions.read().ok().ok_or("Failed to lock")?;
        if let Some(clients) = subscriptions.get(&channel) {
            let users = self.users.read().ok().ok_or("Failed to lock")?;
            let count_ref = &mut count;

            let mut dead_users = Vec::new();
            for client in clients {
                let subscriber = users.get(client).unwrap();
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

            drop(subscriptions);
            drop(users);
            for user in dead_users {
                self.unsubscribe(user)?;
            }
        }
        Ok(count)
    }

    /// Build RESP response
    fn build_response(channel: &str, message: &str) -> String {
        let mut response = ResponseBuilder::new();
        response.add(ProtocolType::Array(vec![
            ProtocolType::String("message".to_string()),
            ProtocolType::String(channel.to_string()),
            ProtocolType::String(message.to_string()),
        ]));
        response.serialize()
    }

    /// Returns the subscriptions list for a specific client
    pub fn get_subscriptions(&self, user: Arc<Client>) -> Result<Vec<String>, &'static str> {
        let users = self.users.read().ok().ok_or("Failed to lock")?;
        return Ok(if let Some(sub) = users.get(&user) {
            sub.channels.iter().cloned().collect::<Vec<String>>()
        } else {
            Vec::new()
        });
    }

    /// Unsubscribes a user from all the channels it's subscribed.
    pub fn unsubscribe_from_channel(
        &self,
        user: Arc<Client>,
        channel: &str,
    ) -> Result<usize, &'static str> {
        let mut users = self.users.write().ok().ok_or("Failed to lock")?;
        let mut subscriptions = self.subscriptions.write().ok().ok_or("Failed to lock")?;
        if let Some(sub) = users.get_mut(&user) {
            if let Some(set) = subscriptions.get_mut(channel) {
                set.remove(&user);
            }
            sub.channels.remove(channel);
            let len = sub.channels.len();
            let is_empty = sub.channels.is_empty();
            drop(subscriptions);
            drop(users);
            if is_empty {
                self.drop_user(user)?;
            }
            Ok(len)
        } else {
            Ok(0)
        }
    }

    /// Unsubscribes a user from all the channels it's subscribed.
    fn unsubscribe(&self, user: Arc<Client>) -> Result<(), &'static str> {
        for channel in self.get_subscriptions(user.clone())? {
            self.unsubscribe_from_channel(user.clone(), &channel)?;
        }
        Ok(())
    }

    /// Return a list of all the active channels
    pub fn get_channels(&self) -> Result<Vec<String>, &'static str> {
        let subs = self.subscriptions.read().ok().ok_or("Failed to lock")?;
        Ok(subs
            .keys()
            .cloned()
            .filter(|x| {
                let count = self.subscriber_count(x);
                count.is_ok() && count.unwrap() > 0
            })
            .collect::<Vec<String>>())
    }

    /// Return the subscriber count for a channel
    pub fn subscriber_count(&self, channel: &str) -> Result<usize, &'static str> {
        let subscriptions = self.subscriptions.read().ok().ok_or("Failed to lock")?;
        return if let Some(s) = subscriptions.get(channel) {
            Ok(s.len())
        } else {
            Ok(0)
        };
    }
}
