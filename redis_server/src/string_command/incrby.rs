use redis_protocol::response::ResponseBuilder;
use redis_protocol::types::ProtocolType;
use crate::storage::data_storage::DataStorage;
use std::sync::Arc;

/// Increments the number stored at key by increment.
/// If the key does not exist, it is set to 0 before performing the operation.
/// An error is returned if the key contains a value of the wrong type or contains a string that can not be represented as integer.
/// This operation is limited to 64 bit signed integers.
pub fn run(
    data: Arc<DataStorage>,
    arguments: Vec<ProtocolType>,
    builder: &mut ResponseBuilder,
) -> Result<(), &'static str> {
    if arguments.len() != 2 {
        return Err("ERR wrong number of arguments for 'incrby' command");
    }

    let key = arguments[0].clone().string()?;
    let number = arguments[1].clone().integer()?;

    match data.increment_value(key, number) {
        Ok(result) => builder.add(ProtocolType::Integer(result)),
        Err(msg) => return Err(msg),
    }

    Ok(())
}

#[cfg(test)]

mod tests {
    use super::*;
    use redis_protocol::types::ProtocolType;
    use crate::storage::data_storage::DataStorage;
    use crate::storage::data_storage::Value;
    use std::sync::Arc;

    #[test]
    fn increment_existing_key() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        data.set("Key", Value::String("10".to_string())).unwrap();

        run(
            data.clone(),
            vec![
                ProtocolType::String("Key".to_string()),
                ProtocolType::String("5".to_string()),
            ],
            &mut builder,
        )
        .unwrap();

        assert_eq!(":15\r\n", builder.serialize());
    }

    #[test]
    fn increment_not_existing_key() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();

        run(
            data.clone(),
            vec![
                ProtocolType::String("Key".to_string()),
                ProtocolType::String("5".to_string()),
            ],
            &mut builder,
        )
        .unwrap();

        assert_eq!(":5\r\n", builder.serialize());
    }

    #[test]
    fn increment_not_integer_key() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        data.set("asdasd", Value::String("value".to_string()))
            .unwrap();

        let result = run(
            data.clone(),
            vec![
                ProtocolType::String("asdasd".to_string()),
                ProtocolType::String("5".to_string()),
            ],
            &mut builder,
        );
        assert!(result.is_err());
    }
}
