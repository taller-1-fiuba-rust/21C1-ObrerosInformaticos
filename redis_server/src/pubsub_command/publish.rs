use crate::pubsub::PublisherSubscriber;
use redis_protocol::response::ResponseBuilder;
use redis_protocol::types::ProtocolType;
use std::sync::Arc;

/// Posts a message to the given channel.
pub fn run(
    pubsub: Arc<PublisherSubscriber>,
    builder: &mut ResponseBuilder,
    arguments: Vec<ProtocolType>,
) -> Result<(), &'static str> {
    if arguments.len() != 2 {
        return Err("Wrong number of arguments");
    }

    let channel = arguments[0].clone().string()?;
    let msg = arguments[1].clone().string()?;

    builder.add(ProtocolType::Integer(pubsub.publish(channel, msg)? as i64));
    Ok(())
}
