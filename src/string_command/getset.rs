use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use crate::storage::data_storage::DataStorage;
use crate::storage::data_storage::Value;
use std::sync::Arc;

pub fn run(
    builder: &mut ResponseBuilder,
    arguments: Vec<ProtocolType>,
    data: &Arc<DataStorage>,
) -> Result<(), &'static str> {
    if arguments.len() != 2 {
        return Err("Wrong quantity of arguments");
    }

    let key = arguments[0].clone().string()?;

    match data.get(&key) {
        Some(value) => {
            match value {
                Value::String(old_value) => {
                    let new_value = arguments[1].clone().string()?;
                    data.set(&key, Value::String(new_value))?;
                    builder.add(ProtocolType::SimpleString(old_value.to_string()));
                },
                Value::Vec(_) => return Err("The value stored is not a string"),
                Value::HashSet(_) => return Err("The value stored is not a string"),
            }
            
        },
        None => builder.add(ProtocolType::SimpleString("nil".to_string()))
    }
    Ok(())
}
