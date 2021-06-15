use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use crate::pubsub::PublisherSubscriber;

use std::sync::{Arc, Mutex};
use crate::client::Client;

/// Execute the pub/sub unsubscribe command.
pub fn run(
    pubsub: Arc<Mutex<PublisherSubscriber>>,
    client: Arc<Client>,
    builder: &mut ResponseBuilder,
    arguments: Vec<ProtocolType>,
) -> Result<(), &'static str> {

    let mut locked_pubsub = pubsub.lock().ok().ok_or("Failed to lock")?;
    let mut channels = arguments.iter().map(|x| x.clone().string().unwrap()).collect::<Vec<String>>();

    if channels.is_empty() {
        channels = locked_pubsub.get_subscriptions(client.clone());
    }

    if channels.is_empty() {
        builder.add(ProtocolType::Array(vec![
            ProtocolType::String("unsubscribe".to_string()),
            ProtocolType::String("none".to_string()),
            ProtocolType::Integer(channels.len() as i64),
        ]));
        return Ok(())
    }

    for channel in channels {
        let current_subs = locked_pubsub.unsubscribe_from_channel(client.clone(), &channel);
        builder.add(ProtocolType::Array(vec![
            ProtocolType::String("unsubscribe".to_string()),
            ProtocolType::String(channel),
            ProtocolType::Integer(current_subs as i64),
        ]));
    }

    Ok(())
}