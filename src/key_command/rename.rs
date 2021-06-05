use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use crate::storage::data_storage::DataStorage;
use std::sync::Arc;

pub fn run(
    db: Arc<DataStorage>,
    arguments: Vec<ProtocolType>,
    builder: &mut ResponseBuilder,
) -> Result<(), &'static str> {
    assert_eq!(arguments.len(), 2);

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
        data.add_key_value("key", Value::String("value".to_string()));

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
        assert_eq!(builder.serialize(), "*1\r\n+OK\r\n");
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
