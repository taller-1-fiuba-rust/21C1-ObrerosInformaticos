use crate::protocol::response::ResponseBuilder;
use crate::pubsub::PublisherSubscriber;
use crate::protocol::types::ProtocolType;
use std::sync::{Mutex, Arc};

pub fn run(pubsub: Arc<Mutex<PublisherSubscriber>>, builder: &mut ResponseBuilder, arguments: Vec<ProtocolType>) -> Result<(), String> {
    assert_eq!(arguments.len(), 2);

    let msg = match arguments[0].clone().string() {
        Ok(s) => s,
        Err(s) => {
            return Err(format!("Error '{}' while parsing message'{}'", s, arguments[0].to_string()));
        }
    };

    let channel = match arguments[1].clone().string() {
      Ok(s) => s,
      Err(s) => {
          return Err(format!("Error '{}' while parsing channel'{}'", s, arguments[1].to_string()));
      }
    };

    let mut locked_pubsub = match pubsub.lock() {
        Err(e) => {
            return Err("Failed to execute publish".to_string());
        },
        Ok(t) => t
    };

    builder.add(ProtocolType::Integer(locked_pubsub.publish(msg, channel) as i32));
    Ok(())
}