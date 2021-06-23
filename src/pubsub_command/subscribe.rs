use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use crate::pubsub::PublisherSubscriber;

use crate::client::Client;
use std::sync::{Arc, Mutex};

/// Execute the pub/sub subscribe command.
pub fn run(
    pubsub: Arc<Mutex<PublisherSubscriber>>,
    client: Arc<Client>,
    builder: &mut ResponseBuilder,
    arguments: Vec<ProtocolType>,
) -> Result<(), &'static str> {
    assert!(!arguments.is_empty());

    let channels = arguments
        .iter()
        .map(|x| x.clone().string().unwrap())
        .collect::<Vec<String>>();

    let mut locked_pubsub = pubsub.lock().ok().ok_or("Failed to lock")?;

    for channel in channels {
        let current_subs = locked_pubsub.subscribe(client.clone(), &channel);
        builder.add(ProtocolType::Array(vec![
            ProtocolType::String("subscribe".to_string()),
            ProtocolType::String(channel),
            ProtocolType::Integer(current_subs as i64),
        ]));
    }

    Ok(())
}
