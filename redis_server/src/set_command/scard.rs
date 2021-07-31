use redis_protocol::response::ResponseBuilder;
use redis_protocol::types::ProtocolType;
use crate::storage::data_storage::DataStorage;
use crate::storage::data_storage::Value;
use std::sync::Arc;

/// Returns the set cardinality (number of elements) of the set stored at key.
pub fn run(
    builder: &mut ResponseBuilder,
    arguments: Vec<ProtocolType>,
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
            Value::Vec(_) => {
                return Err("WRONGTYPE Operation against a key holding the wrong kind of value")
            }
            Value::HashSet(set) => {
                builder.add(ProtocolType::Integer(set.len() as i64));
            }
        },

        None => builder.add(ProtocolType::Integer(0)),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use redis_protocol::types::ProtocolType;
    use crate::storage::data_storage::DataStorage;
    use std::collections::HashSet;
    use std::sync::Arc;

    #[test]
    fn test_llen_of_list_is_ok() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        let mut set = HashSet::new();
        set.insert("value1".to_string());
        set.insert("value2".to_string());
        data.set("test", Value::HashSet(set)).unwrap();

        run(
            &mut builder,
            vec![ProtocolType::String("test".to_string())],
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
            &mut builder,
            vec![ProtocolType::String("test".to_string())],
            data.clone(),
        )
        .unwrap();

        assert_eq!(":0\r\n", builder.serialize());
    }

    #[test]
    fn test_llen_of_hashset_returns_wrongtype() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        data.set("test", Value::Vec(Vec::new())).unwrap();

        let result = run(
            &mut builder,
            vec![ProtocolType::String("test".to_string())],
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
            &mut builder,
            vec![ProtocolType::String("test".to_string())],
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
