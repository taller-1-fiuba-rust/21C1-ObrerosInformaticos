use crate::storage::data_storage::DataStorage;
use redis_protocol::response::ResponseBuilder;
use redis_protocol::types::ProtocolType;
use std::sync::Arc;

/// Renames key to newkey. It returns an error when key does not exist. If newkey already exists it is overwritten.
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

    db.rename(&src, &dst)?;
    builder.add(ProtocolType::SimpleString("OK".to_string()));
    Ok(())
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::storage::data_storage::Value;

    #[test]
    fn test_rename() {
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
        assert!(data.get("key").is_none());
        assert_eq!(builder.serialize(), "+OK\r\n");
    }

    #[test]
    #[should_panic]
    fn test_rename_error() {
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
    }
}
