use crate::storage::data_storage::DataStorage;
use crate::storage::data_storage::Value;
use redis_protocol::response::ResponseBuilder;
use redis_protocol::types::ProtocolType;
use std::sync::Arc;
use std::usize;

/// Returns the element at index index in the list stored at key.
/// The index is zero-based, so 0 means the first element, 1 the second element and so on.
/// Negative indices can be used to designate elements starting at the tail of the list.
/// Here, -1 means the last element, -2 means the penultimate and so forth.
/// When the value at key is not a list, an error is returned.
pub fn run(
    arguments: Vec<ProtocolType>,
    builder: &mut ResponseBuilder,
    data: Arc<DataStorage>,
) -> Result<(), &'static str> {
    if arguments.len() != 2 {
        return Err("ERR wrong number of arguments for 'lindex' command");
    }
    let string_key = arguments[0].clone().string()?;
    let string_index = arguments[1].clone().string()?;
    let i8_index: i8 = match string_index.parse() {
        Ok(numb) => numb,
        Err(_) => {
            builder.add(ProtocolType::Nil());
            return Ok(());
        }
    };
    let result = data.get(&string_key);
    match result {
        Some(value) => match value {
            Value::String(_) => {
                return Err("WRONGTYPE Operation against a key holding the wrong kind of value")
            }
            Value::Vec(list) => {
                let usize_index;
                if i8_index >= 0 {
                    usize_index = i8_index as usize;
                } else {
                    let positive_index = (i8_index - i8_index * 2) as usize;
                    if positive_index > list.len() {
                        usize_index = positive_index;
                    } else {
                        usize_index = list.len() - positive_index;
                    }
                }
                let element = list.get(usize_index as usize);
                match element {
                    Some(res) => builder.add(ProtocolType::String(res.to_string())),
                    None => builder.add(ProtocolType::Nil()),
                }
            }
            Value::HashSet(_) => {
                return Err("WRONGTYPE Operation against a key holding the wrong kind of value")
            }
        },

        None => builder.add(ProtocolType::Nil()),
    }
    Ok(())
}

#[cfg(test)]

mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_lindex_of_vec_returns_ok() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        data.set(
            "test",
            Value::Vec(["value1".to_string(), "value2".to_string()].to_vec()),
        )
        .unwrap();

        run(
            vec![
                ProtocolType::String("test".to_string()),
                ProtocolType::String("1".to_string()),
            ],
            &mut builder,
            data.clone(),
        )
        .unwrap();

        assert_eq!("$6\r\nvalue2\r\n", builder.serialize());
    }

    #[test]
    fn test_lindex_of_string_returns_wrongtype() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        data.set("test", Value::String("value".to_string()))
            .unwrap();

        let result = run(
            vec![
                ProtocolType::String("test".to_string()),
                ProtocolType::String("1".to_string()),
            ],
            &mut builder,
            data.clone(),
        );

        match result {
            Ok(_) => assert_eq!(true, false),
            Err(msg) => assert_eq!(
                msg,
                "WRONGTYPE Operation against a key holding the wrong kind of value"
            ),
        }
    }

    #[test]
    fn test_lindex_of_hashset_returns_wrongtype() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        data.set("test", Value::HashSet(HashSet::new())).unwrap();

        let result = run(
            vec![
                ProtocolType::String("test".to_string()),
                ProtocolType::String("1".to_string()),
            ],
            &mut builder,
            data.clone(),
        );

        match result {
            Ok(_) => assert_eq!(true, false),
            Err(msg) => assert_eq!(
                msg,
                "WRONGTYPE Operation against a key holding the wrong kind of value"
            ),
        }
    }

    #[test]
    fn test_lindex_of_nothing_returns_nil() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();

        run(
            vec![
                ProtocolType::String("test".to_string()),
                ProtocolType::String("1".to_string()),
            ],
            &mut builder,
            data.clone(),
        )
        .unwrap();

        assert_eq!("$-1\r\n", builder.serialize());
    }

    #[test]
    fn test_overflow_of_vec_returns_nil() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        data.set(
            "test",
            Value::Vec(["value1".to_string(), "value2".to_string()].to_vec()),
        )
        .unwrap();

        run(
            vec![
                ProtocolType::String("test".to_string()),
                ProtocolType::String("3".to_string()),
            ],
            &mut builder,
            data.clone(),
        )
        .unwrap();

        assert_eq!("$-1\r\n", builder.serialize());
    }

    #[test]
    fn test_lindex_with_negative_returns_ok() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        data.set(
            "test",
            Value::Vec(["value1".to_string(), "value2".to_string()].to_vec()),
        )
        .unwrap();

        run(
            vec![
                ProtocolType::String("test".to_string()),
                ProtocolType::String("-1".to_string()),
            ],
            &mut builder,
            data.clone(),
        )
        .unwrap();

        assert_eq!("$6\r\nvalue2\r\n", builder.serialize());
    }
}
