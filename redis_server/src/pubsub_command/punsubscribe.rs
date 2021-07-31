use redis_protocol::response::ResponseBuilder;
use redis_protocol::types::ProtocolType;
use crate::pubsub::PublisherSubscriber;

use crate::client::Client;
use std::sync::Arc;

///Unsubscribes the client from the given patterns, or from all of them if none is given.
pub fn run(
    pubsub: Arc<PublisherSubscriber>,
    client: Arc<Client>,
    builder: &mut ResponseBuilder,
) -> Result<(), &'static str> {
    builder.add(ProtocolType::Array(vec![
        ProtocolType::String("punsubscribe".to_string()),
        ProtocolType::String("none".to_string()),
        ProtocolType::Integer(pubsub.get_subscriptions(client)?.len() as i64),
    ]));

    Ok(())
}
