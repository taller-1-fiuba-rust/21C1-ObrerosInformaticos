use crate::protocol::types::ProtocolType;
use crate::storage::data_storage::DataStorage;
use crate::storage::data_storage::Value;
use crate::protocol::response::ResponseBuilder;
use std::sync::Arc;


pub fn run(
    arguments: Vec<ProtocolType>,
    builder: &mut ResponseBuilder,
    data: Arc<DataStorage>,
) -> Result<(), &'static str> {

    if arguments.len() > 2 {
        return Err("ERR wrong number of arguments");
    }
    let string_key = arguments[0].clone().string()?;
    let int_index = arguments[1].clone().integer()?;
    let result = data.get(&string_key);

    match result {
        Some(value) => match value {
            Value::String(_) => response.push(ProtocolType::String(string)),
            Value::Vec(list) => response.push(ProtocolType::String("nil".to_string())),
            Value::HashSet(_) => response.push(ProtocolType::String("nil".to_string())),
        },
        None => response.push(ProtocolType::String("nil".to_string())),
    }
    }

    builder.add(ProtocolType::Array(response));
    Ok(())
}

#[cfg(test)]

mod tests {
    // use super::*;
    // use crate::protocol::types::ProtocolType;
    // use crate::storage::data_storage::DataStorage;
    // use crate::storage::data_storage::Value;
    // use std::sync::Arc;

    // #[test]
    // fn insert_one_key() {
    //     let data = Arc::new(DataStorage::new());
    //     let mut builder = ResponseBuilder::new();
    //     data.set("Test", Value::Vec(["value".to_string()].to_vec()))
    //         .unwrap();

    //     run(
    //         &mut builder,
    //         vec![
    //             ProtocolType::String("Test".to_string()),
    //             ProtocolType::String("value2".to_string()),
    //         ],
    //         data.clone(),
    //     )
    //     .unwrap();

    //     assert_eq!(":2\r\n", builder.serialize());
    // }

    // #[test]
    // fn insert_keys() {
    //     let data = Arc::new(DataStorage::new());
    //     let mut builder = ResponseBuilder::new();
    //     data.set("Test", Value::Vec(["1".to_string()].to_vec()))
    //         .unwrap();

    //     run(
    //         &mut builder,
    //         vec![
    //             ProtocolType::String("Test".to_string()),
    //             ProtocolType::String("2".to_string()),
    //             ProtocolType::String("3".to_string()),
    //             ProtocolType::String("4".to_string()),
    //         ],
    //         data.clone(),
    //     )
    //     .unwrap();

    //     assert_eq!(":4\r\n", builder.serialize());
    // }

    // #[test]
    // fn insert_to_a_not_existing_key() {
    //     let data = Arc::new(DataStorage::new());
    //     let mut builder = ResponseBuilder::new();

    //     run(
    //         &mut builder,
    //         vec![
    //             ProtocolType::String("1".to_string()),
    //             ProtocolType::String("2".to_string()),
    //         ],
    //         data.clone(),
    //     )
    //     .unwrap();

    //     assert_eq!(":0\r\n", builder.serialize());
    // }
}
