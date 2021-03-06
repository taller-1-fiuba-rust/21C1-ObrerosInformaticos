use crate::pubsub::PublisherSubscriber;
use redis_protocol::response::ResponseBuilder;
use redis_protocol::types::ProtocolType;

use crate::client::Client;
use std::sync::Arc;

/// Execute the pub/sub subscribe command.
pub fn run(
    pubsub: Arc<PublisherSubscriber>,
    client: Arc<Client>,
    builder: &mut ResponseBuilder,
    arguments: Vec<ProtocolType>,
) -> Result<(), &'static str> {
    if arguments.is_empty() {
        return Err("Wrong number of arguments");
    }

    let channels = arguments
        .iter()
        .map(|x| x.clone().string().unwrap())
        .collect::<Vec<String>>();

    for channel in channels {
        let current_subs = pubsub.subscribe(client.clone(), &channel)?;
        builder.add(ProtocolType::Array(vec![
            ProtocolType::String("subscribe".to_string()),
            ProtocolType::String(channel),
            ProtocolType::Integer(current_subs as i64),
        ]));
    }

    Ok(())
}
