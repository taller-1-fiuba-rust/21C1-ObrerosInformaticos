use crate::protocol::command::Command;
use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use crate::storage::data_storage::DataStorage;
use std::sync::Arc;

pub fn run(
    arguments: Vec<ProtocolType>,
    builder: &mut ResponseBuilder,
    data: &Arc<DataStorage>,
) -> Result<(), &'static str> {
    if arguments.len() != 1 {
        return Err("Wrong quantity of arguments. Command TYPE has only one.")
    }

    let key = arguments[0].clone().string();

    println!("{}", key.unwrap());
    Ok(())
}