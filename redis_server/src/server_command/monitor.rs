use crate::client::Client;
use crate::monitor::Monitor;
use redis_protocol::response::ResponseBuilder;
use redis_protocol::types::ProtocolType;
use std::sync::Arc;

///Add client to monitor.
pub fn run(
    monitor: &Monitor,
    client: Arc<Client>,
    builder: &mut ResponseBuilder,
) -> Result<(), &'static str> {
    monitor.add(client)?;
    builder.add(ProtocolType::SimpleString("OK".to_string()));
    Ok(())
}
