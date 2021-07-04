use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use crate::storage::data_storage::DataStorage;
use crate::storage::data_storage::Value;
use std::sync::Arc;

pub fn run(
    arguments: Vec<ProtocolType>,
    builder: &mut ResponseBuilder,
    data: Arc<DataStorage>,
) -> Result<(), &'static str> {
    if arguments.len() != 1 {
        return Err("ERR wrong number of arguments");
    }
    let string_key = arguments[0].clone().string()?;
    let result = data.get(&string_key);
    match result {
        Some(value) => match value {
            Value::String(_) => {
                return Err("WRONGTYPE Operation against a key holding the wrong kind of value")
            }
            Value::Vec(list) => {
                builder.add(ProtocolType::Integer(list.len() as i64));
            }
            Value::HashSet(_) => {
                return Err("WRONGTYPE Operation against a key holding the wrong kind of value")
            }
        },

        None => builder.add(ProtocolType::Integer(0)),
    }
    Ok(())
}

#[cfg(test)]

mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_llen_of_list_is_ok() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        data.set(
            "test",
            Value::Vec(["value1".to_string(), "value2".to_string()].to_vec()),
        )
        .unwrap();

        run(
            vec![ProtocolType::String("test".to_string())],
            &mut builder,
            data.clone(),
        )
        .unwrap();

        assert_eq!(":2\r\n", builder.serialize());
    }

    #[test]
    fn test_llen_of_unexistent_key_returns_0() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();

        run(
            vec![ProtocolType::String("test".to_string())],
            &mut builder,
            data.clone(),
        )
        .unwrap();

        assert_eq!(":0\r\n", builder.serialize());
    }

    #[test]
    fn test_llen_of_hashset_returns_wrongtype() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        data.set("test", Value::HashSet(HashSet::new())).unwrap();

        let result = run(
            vec![ProtocolType::String("test".to_string())],
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
    fn test_llen_of_string_returns_wrongtype() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        data.set("test", Value::String("".to_string())).unwrap();

        let result = run(
            vec![ProtocolType::String("test".to_string())],
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
}
