use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use crate::pubsub::PublisherSubscriber;
use std::sync::{Arc, Mutex};

/// Execute the pub/sub publish command.
pub fn run(
    pubsub: Arc<Mutex<PublisherSubscriber>>,
    builder: &mut ResponseBuilder,
    arguments: Vec<ProtocolType>,
) -> Result<(), &'static str> {
    assert_eq!(arguments.len(), 2);

    let channel = arguments[0].clone().string()?;
    let msg = arguments[1].clone().string()?;

    let mut locked_pubsub = pubsub.lock().ok().ok_or("Failed to lock")?;

    builder.add(ProtocolType::Integer(
        locked_pubsub.publish(channel, msg) as i64
    ));
    Ok(())
}
