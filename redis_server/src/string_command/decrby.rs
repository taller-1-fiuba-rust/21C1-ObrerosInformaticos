use crate::storage::data_storage::DataStorage;
use redis_protocol::response::ResponseBuilder;
use redis_protocol::types::ProtocolType;
use std::sync::Arc;

/// Decrements the number stored at key by decrement.
/// If the key does not exist, it is set to 0 before performing the operation.
/// An error is returned if the key contains a value of the wrong type or contains
/// a string that can not be represented as integer.
pub fn run(
    data: Arc<DataStorage>,
    arguments: Vec<ProtocolType>,
    builder: &mut ResponseBuilder,
) -> Result<(), &'static str> {
    if arguments.len() != 2 {
        return Err("Wrong number of arguments");
    }

    let key = arguments[0].clone().string()?;
    let number = arguments[1].clone().integer()?;

    match data.decrement_value(key, number) {
        Ok(s) => builder.add(ProtocolType::Integer(s as i64)),
        Err(j) => builder.add(ProtocolType::Error(j.to_string())),
    }

    Ok(())
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::storage::data_storage::DataStorage;
    use crate::storage::data_storage::Value;
    use redis_protocol::types::ProtocolType;
    use std::sync::Arc;

    #[test]
    fn decrement_existing_key() {
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

        assert_eq!(":5\r\n", builder.serialize());
    }

    #[test]
    fn decrement_not_existing_key() {
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

        assert_eq!(":-5\r\n", builder.serialize());
    }

    #[test]
    fn decrement_not_integer_key() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        data.set("Key", Value::String("value".to_string())).unwrap();

        run(
            data.clone(),
            vec![
                ProtocolType::String("Key".to_string()),
                ProtocolType::String("5".to_string()),
            ],
            &mut builder,
        )
        .unwrap();

        assert_eq!(
            "-Cant decrement a value to a not integer value\r\n",
            builder.serialize()
        );
    }
}
