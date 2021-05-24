use std::sync::Arc;
use crate::protocol::response::ResponseBuilder;
use crate::protocol::command::Command;
use crate::storage::data_storage::DataStorage;
use crate::protocol::types::ProtocolType;

pub fn run(cmd: &Command, builder: &mut ResponseBuilder, data: &Arc<DataStorage>){
	//builde.add(ProtocolType::String("PONG".to_string())
}