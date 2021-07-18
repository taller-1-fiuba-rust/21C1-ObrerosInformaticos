use crate::client::Client;
use crate::monitor::Monitor;
use std::sync::Arc;

///Add client to monitor.
pub fn run(
    monitor: &Monitor,
    client: Arc<Client>,
) -> Result<(), &'static str> {
    monitor.add(client)?;
    Ok(())
}