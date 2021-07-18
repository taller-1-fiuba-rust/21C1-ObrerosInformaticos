use crate::client::Client;
use crate::monitor::Monitor;
use std::sync::Arc;
use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;

///Add client to monitor.
pub fn run(monitor: &Monitor, client: Arc<Client>, builder: &mut ResponseBuilder) -> Result<(), &'static str> {
    monitor.add(client)?;
    builder.add(ProtocolType::SimpleString("OK".to_string()));
    Ok(())
}
