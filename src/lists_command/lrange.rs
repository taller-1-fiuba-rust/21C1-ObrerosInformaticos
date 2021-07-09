use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use crate::storage::data_storage::DataStorage;
use std::sync::Arc;

pub fn run(
    builder: &mut ResponseBuilder,
    arguments: Vec<ProtocolType>,
    data: Arc<DataStorage>,
) -> Result<(), &'static str> {
    if arguments.len() != 3 {
        return Err("Wrong quantity of arguments.");
    }

    let key = arguments[0].clone().string()?;
    let first_index = arguments[1].clone().integer()?;
    let second_index = arguments[2].clone().integer()?;

    let values = data.lrange(key, first_index, second_index);

    match values {
        Ok(val) => 
            match val {
                Some(vec_values) => {
                    builder.add(ProtocolType::Array(vec_values.into_iter().map(ProtocolType::String).collect(),));
                    Ok(())
                },
                None => {
                    builder.add(ProtocolType::String("(empty list)".to_string()));
                    Ok(())
                }
            }   
        Err(s) => {
            builder.add(ProtocolType::Error(s.to_string()));
            Err(s)
        }
    }
}
