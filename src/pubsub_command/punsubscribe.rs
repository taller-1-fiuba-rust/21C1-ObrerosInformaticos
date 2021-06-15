use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use crate::pubsub::PublisherSubscriber;

use std::sync::{Arc, Mutex};
use crate::client::Client;

pub fn run(
    pubsub: Arc<Mutex<PublisherSubscriber>>,
    client: Arc<Client>,
    builder: &mut ResponseBuilder,
    arguments: Vec<ProtocolType>,
) -> Result<(), &'static str> {

    let lock = pubsub.lock().ok().ok_or_else(|| "Error locking pubsub")?;

    builder.add(ProtocolType::Array(
        vec![ProtocolType::String("punsubscribe".to_string()),
            ProtocolType::String("none".to_string()),
            ProtocolType::Integer(lock.get_subscriptions(client).len() as i64)]));

    Ok(())
}