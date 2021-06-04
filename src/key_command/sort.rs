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

    if arguments.len() == 1 {
        return basic_sort(builder, arguments, data)
    }

    Ok(())
}

fn basic_sort(
    builder: &mut ResponseBuilder,
    arguments: Vec<ProtocolType>,
    data: &Arc<DataStorage>,
) -> Result<(), &'static str> {

    let key = arguments[0].clone().string()?;

    let values = data.get(&key);

    match values {
        None => return Err("None"),
        Some(Value::String(string)) => return Err("String value. No possible sort."),
        Some(Value::Vec(vecs)) => {
            // vec.sort();
            builder.add(ProtocolType::Array(vecs));
        },
        Some(Value::HashSet(set)) => {
            builder.add(ProtocolType::Array(set));
        },
    }
    Ok(())
}