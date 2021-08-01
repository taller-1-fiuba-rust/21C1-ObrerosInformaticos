use crate::storage::data_storage::DataStorage;
use redis_protocol::response::ResponseBuilder;
use redis_protocol::types::ProtocolType;
use std::sync::Arc;

/// This command copies the value stored at the source key to the destination key.
pub fn run(
    db: Arc<DataStorage>,
    arguments: Vec<ProtocolType>,
    builder: &mut ResponseBuilder,
) -> Result<(), &'static str> {
    if arguments.len() != 2 {
        return Err("Wrong number of arguments");
    }

    let src = arguments[0].clone().string()?;
    let dst = arguments[1].clone().string()?;

    let option = db.get(&src);
    let mut result = 0;
    if let Some(value) = option {
        db.set(&dst, value)?;
        result = 1;
    }

    builder.add(ProtocolType::Integer(result));
    Ok(())
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::storage::data_storage::Value;

    #[test]
    fn test_copy() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        data.set("key", Value::String("value".to_string())).unwrap();

        run(
            data.clone(),
            vec![
                ProtocolType::String("key".to_string()),
                ProtocolType::String("new_key".to_string()),
            ],
            &mut builder,
        )
        .unwrap();

        assert_eq!(data.get("new_key").unwrap().string().unwrap(), "value");
        assert!(data.get("key").is_some());
        assert_eq!(builder.serialize(), ":1\r\n");
    }

    #[test]
    fn test_copy_with_empty_element() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();

        run(
            data,
            vec![
                ProtocolType::String("no_such_key".to_string()),
                ProtocolType::String("new_key".to_string()),
            ],
            &mut builder,
        )
        .unwrap();

        assert_eq!(builder.serialize(), ":0\r\n");
    }
}
