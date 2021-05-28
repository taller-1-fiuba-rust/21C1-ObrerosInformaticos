use crate::protocol::command::Command;
use crate::protocol::response::ResponseBuilder;
use crate::storage::data_storage::DataStorage;
use crate::protocol::types::ProtocolType;
use std::time::SystemTime;
use std::time::Duration;
use std::sync::Arc;

pub fn set_expiration_to_key(builder: &mut ResponseBuilder, cmd: &Command, data: &Arc<DataStorage>) -> Result<(), &'static str> {

	let arguments: Vec<ProtocolType> = cmd.arguments();

	//data.set_expiration_to_key(SystemTime::now(), Duration::from_secs(seconds), key);

	Ok(())
}