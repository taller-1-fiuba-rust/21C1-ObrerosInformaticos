use crate::client::Client;
use crate::monitor::Monitor;
use std::sync::Arc;

pub fn run(
    monitor: &Monitor,
    client: Arc<Client>,
) -> Result<(), &'static str> {
    monitor.add(client)?;
    Ok(())
}