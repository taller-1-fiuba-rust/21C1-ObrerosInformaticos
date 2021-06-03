use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use crate::storage::data_storage::DataStorage;
use crate::storage::data_storage::Value;
use std::sync::Arc;

pub fn run(
    arguments: Vec<ProtocolType>,
    builder: &mut ResponseBuilder,
    data: &Arc<DataStorage>,
) -> Result<(), &'static str> {
    if arguments.len() != 1 {
        return Err("Wrong quantity of arguments. Command TYPE has only one.");
    }

    let key = arguments[0].clone().string()?;

    let value_option = data.get(&key);

    if let Some(value) = value_option {
        match value {
            Value::String(_) => builder.add(ProtocolType::String("String".to_string())),
            Value::Vec(_) => builder.add(ProtocolType::String("Vec".to_string())),
            Value::HashSet(_) => builder.add(ProtocolType::String("HashSet".to_string())),
        }
    } else {
        return Err("There's no value for that key");
    }
    Ok(())
}
