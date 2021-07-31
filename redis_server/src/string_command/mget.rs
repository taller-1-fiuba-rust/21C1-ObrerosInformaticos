use redis_protocol::response::ResponseBuilder;
use redis_protocol::types::ProtocolType;
use crate::storage::data_storage::DataStorage;
use crate::storage::data_storage::Value;
use std::sync::Arc;

/// Returns the values of all specified keys.
/// For every key that does not hold a string value or does not exist, the special value nil is returned.
/// Because of this, the operation never fails.
pub fn run(
    arguments: Vec<ProtocolType>,
    builder: &mut ResponseBuilder,
    data: Arc<DataStorage>,
) -> Result<(), &'static str> {
    let mut response = Vec::new();
    for key in arguments.iter() {
        let string_key = key.clone().string()?;
        let result = data.get(&string_key);

        match result {
            Some(value) => match value {
                Value::String(string) => response.push(ProtocolType::String(string)),
                Value::Vec(_) => response.push(ProtocolType::Nil()),
                Value::HashSet(_) => response.push(ProtocolType::Nil()),
            },
            None => response.push(ProtocolType::Nil()),
        }
    }

    builder.add(ProtocolType::Array(response));
    Ok(())
}

#[cfg(test)]

mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_gets_key1_and_2_nils() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();

        data.set("key1", Value::String("value".to_string()))
            .unwrap();
        data.set("key2", Value::Vec(vec![])).unwrap();

        run(
            vec![
                ProtocolType::String("key1".to_string()),
                ProtocolType::String("key2".to_string()),
                ProtocolType::String("XX".to_string()),
            ],
            &mut builder,
            data,
        )
        .unwrap();

        assert_eq!(builder.serialize(), "*3\r\n$5\r\nvalue\r\n$-1\r\n$-1\r\n");
    }

    #[test]
    fn test_gets_nil_from_hasmap() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();

        data.set("key2", Value::HashSet(HashSet::new())).unwrap();

        run(
            vec![ProtocolType::String("key1".to_string())],
            &mut builder,
            data,
        )
        .unwrap();

        assert_eq!(builder.serialize(), "*1\r\n$-1\r\n");
    }

    #[test]
    fn test_gets_multiple_values() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();

        data.set("key1", Value::String("value".to_string()))
            .unwrap();
        data.set("key2", Value::String("value2".to_string()))
            .unwrap();

        run(
            vec![
                ProtocolType::String("key1".to_string()),
                ProtocolType::String("key2".to_string()),
            ],
            &mut builder,
            data,
        )
        .unwrap();

        assert_eq!(builder.serialize(), "*2\r\n$5\r\nvalue\r\n$6\r\nvalue2\r\n");
    }
}
