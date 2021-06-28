use crate::protocol::types::ProtocolType;
use crate::storage::data_storage::DataStorage;
use crate::storage::data_storage::Value;
use crate::protocol::response::ResponseBuilder;
use std::sync::Arc;
use std::usize;


pub fn run(
    arguments: Vec<ProtocolType>,
    builder: &mut ResponseBuilder,
    data: Arc<DataStorage>,
) -> Result<(), &'static str> {


    data.set("asd", Value::Vec(vec!["asd".to_string(), "ola".to_string()]))?;
    if arguments.len() != 2 {
        return Err("ERR wrong number of arguments");
    }
    let string_key = arguments[0].clone().string()?;
    let string_index = arguments[1].clone().string()?;
    let i8_index: i8 = match string_index.parse() {
        Ok(numb) => numb,
        Err(_) => {
            builder.add(ProtocolType::String("nil".to_string()));
            return Ok(())
        }
    };
    let result = data.get(&string_key);

    match result {
        Some(value) => match value {
            Value::String(_) => return Err("WRONGTYPE Operation against a key holding the wrong kind of value"),
            Value::Vec(list) => {
                if i8_index >= 0 {
                    let element = list.get(i8_index as usize);
                    match element{
                        Some(res) =>  builder.add(ProtocolType::String(res.to_string())),
                        None => builder.add(ProtocolType::String("nil".to_string())),
                    }
                }
                for i in 0..list.len() - 1{
                    if i as i8 == list.len() as i8 + i8_index {
                        builder.add(ProtocolType::String(list[i].clone()));
                        return Ok(())
                    }
                }
                
            },
            Value::HashSet(_) => return Err("WRONGTYPE Operation against a key holding the wrong kind of value"),
        },
        None => builder.add(ProtocolType::String("nil".to_string())),
    }
    
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
