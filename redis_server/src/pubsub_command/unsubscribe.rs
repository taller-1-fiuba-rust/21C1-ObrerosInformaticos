use crate::pubsub::PublisherSubscriber;
use redis_protocol::response::ResponseBuilder;
use redis_protocol::types::ProtocolType;

use crate::client::Client;
use std::sync::Arc;

/// Execute the pub/sub unsubscribe command.
pub fn run(
    pubsub: Arc<PublisherSubscriber>,
    client: Arc<Client>,
    builder: &mut ResponseBuilder,
    arguments: Vec<ProtocolType>,
) -> Result<(), &'static str> {
    let mut channels = arguments
        .iter()
        .map(|x| x.clone().string().unwrap())
        .collect::<Vec<String>>();

    if channels.is_empty() {
        channels = pubsub.get_subscriptions(client.clone())?;
    }

    if channels.is_empty() {
        builder.add(ProtocolType::Array(vec![
            ProtocolType::String("unsubscribe".to_string()),
            ProtocolType::String("none".to_string()),
            ProtocolType::Integer(channels.len() as i64),
        ]));
        return Ok(());
    }

    for channel in channels {
        let current_subs = pubsub.unsubscribe_from_channel(client.clone(), &channel)?;
        builder.add(ProtocolType::Array(vec![
            ProtocolType::String("unsubscribe".to_string()),
            ProtocolType::String(channel),
            ProtocolType::Integer(current_subs as i64),
        ]));
    }

    Ok(())
}
