use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use crate::pubsub::PublisherSubscriber;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

/// Execute the pub/sub subscribe command.
pub fn run(
    pubsub: Arc<Mutex<PublisherSubscriber>>,
    client: Arc<Mutex<TcpStream>>,
    builder: &mut ResponseBuilder,
    arguments: Vec<ProtocolType>,
) -> Result<(), String> {
    assert!(!arguments.is_empty());

    let channels = match parse_channels(&arguments) {
        Ok(c) => c,
        Err(s) => {
            return Err(s);
        }
    };

    let mut locked_pubsub = match pubsub.lock() {
        Err(_) => {
            return Err("Failed to execute subscribe".to_string());
        }
        Ok(t) => t,
    };

    for channel in channels {
        let current_subs = locked_pubsub.subscribe(client.clone(), &channel);
        builder.add(ProtocolType::Array(vec![
            ProtocolType::String("subscribe".to_string()),
            ProtocolType::String(channel),
            ProtocolType::Integer(current_subs as i32),
        ]));
    }

    Ok(())
}

fn parse_channels(arguments: &[ProtocolType]) -> Result<Vec<String>, String> {
    let mut channels = Vec::new();
    for argument in arguments {
        let channel = match (*argument).clone().string() {
            Ok(s) => s,
            Err(s) => {
                return Err(format!(
                    "Error '{}' while parsing channel'{}'",
                    s,
                    arguments[1].to_string()
                ));
            }
        };
        channels.push(channel);
    }
    Ok(channels)
}
