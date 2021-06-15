use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use crate::pubsub::PublisherSubscriber;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use crate::client::Client;

/// Execute the pub/sub subscribe command.
pub fn run(
    pubsub: Arc<Mutex<PublisherSubscriber>>,
    client: Arc<Client>,
    builder: &mut ResponseBuilder,
    arguments: Vec<ProtocolType>,
) -> Result<(), &'static str> {
    assert!(!arguments.is_empty());

    let channels = parse_channels(&arguments)?;

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

fn parse_channels(arguments: &[ProtocolType]) -> Result<Vec<String>, &'static str> {
    let mut channels = Vec::new();
    for argument in arguments {
        let channel = match (*argument).clone().string() {
            Ok(s) => s,
            Err(_) => {
                return Err("Error while parsing channels");
            }
        };
        channels.push(channel);
    }
    Ok(channels)
}
