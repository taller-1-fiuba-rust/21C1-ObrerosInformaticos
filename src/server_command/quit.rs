use crate::client::Client;
use crate::monitor::Monitor;
use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use std::sync::Arc;

///Add client to monitor.
pub fn run(
    monitor: &Monitor,
    client: Arc<Client>,
    builder: &mut ResponseBuilder,
) -> Result<(), &'static str> {
    monitor.remove(client)?;
    builder.add(ProtocolType::SimpleString("OK".to_string()));
    Ok(())
}
