use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use crate::pubsub::PublisherSubscriber;
use std::sync::{Arc, Mutex};

/// Execute the pub/sub publish command.
pub fn run(
    pubsub: Arc<Mutex<PublisherSubscriber>>,
    builder: &mut ResponseBuilder,
    arguments: Vec<ProtocolType>,
) -> Result<(), String> {
    assert_eq!(arguments.len(), 2);

    let channel = match arguments[0].clone().string() {
        Ok(s) => s,
        Err(s) => {
            return Err(format!(
                "Error '{}' while parsing channel'{}'",
                s,
                arguments[0].to_string()
            ));
        }
    };

    let msg = match arguments[1].clone().string() {
        Ok(s) => s,
        Err(s) => {
            return Err(format!(
                "Error '{}' while parsing message'{}'",
                s,
                arguments[1].to_string()
            ));
        }
    };

    let mut locked_pubsub = match pubsub.lock() {
        Err(_) => {
            return Err("Failed to execute publish".to_string());
        }
        Ok(t) => t,
    };

    builder.add(ProtocolType::Integer(
        locked_pubsub.publish(channel, msg) as i32
    ));
    Ok(())
}
