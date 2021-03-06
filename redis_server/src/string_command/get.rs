use crate::storage::data_storage::DataStorage;
use redis_protocol::response::ResponseBuilder;
use redis_protocol::types::ProtocolType;
use std::sync::Arc;

/// Get the value of key. If the key does not exist the special value nil is returned.
/// An error is returned if the value stored at key is not a string,
pub fn run(
    arguments: Vec<ProtocolType>,
    builder: &mut ResponseBuilder,
    data: Arc<DataStorage>,
) -> Result<(), &'static str> {
    if arguments.len() != 1 {
        return Err("ERR wrong number of arguments for 'get' command");
    }

    let key = arguments[0].clone().string()?;

    let value = data.get_string_value(key);

    match value {
        Ok(s) => match s {
            Some(value) => {
                builder.add(ProtocolType::String(value));
                Ok(())
            }
            None => {
                builder.add(ProtocolType::Nil());
                Ok(())
            }
        },
        Err(_i) => Err("Key value not a string"),
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::storage::data_storage::Value;

    #[test]
    fn test_get_existing_key() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        data.set("key", Value::String("Value".to_string())).unwrap();

        run(
            vec![ProtocolType::String("key".to_string())],
            &mut builder,
            data.clone(),
        )
        .unwrap();

        assert_eq!(builder.serialize(), "$5\r\nValue\r\n");
    }

    #[test]
    fn test_get_not_existing_key() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();

        run(
            vec![ProtocolType::String("key".to_string())],
            &mut builder,
            data.clone(),
        )
        .unwrap();

        assert_eq!(builder.serialize(), "$-1\r\n");
    }
}
