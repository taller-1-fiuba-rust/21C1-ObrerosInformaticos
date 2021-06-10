use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use crate::storage::data_storage::DataStorage;
use std::sync::Arc;

pub fn run(
    db: Arc<DataStorage>,
    arguments: Vec<ProtocolType>,
    builder: &mut ResponseBuilder,
) -> Result<(), &'static str> {
    assert_eq!(arguments.len(), 1);

    let src = arguments[0].clone().string()?;

    let mut result = 0;
    let (duration_maybe, _) = db.get_with_expiration(&src).ok_or("Key not found")?;
    if duration_maybe.is_some() {
        result = 1;
        db.set_expiration_to_key(None, &src)?;
    }
    builder.add(ProtocolType::Integer(result));
    Ok(())
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::storage::data_storage::Value;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    #[test]
    fn test_persist() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        let expiration_time = SystemTime::now()
            .checked_add(Duration::from_secs(10))
            .unwrap()
            .duration_since(UNIX_EPOCH)
            .unwrap();
        data.add_with_expiration("src", Value::String("value".to_string()), expiration_time)
            .unwrap();

        run(
            data.clone(),
            vec![ProtocolType::String("src".to_string())],
            &mut builder,
        )
        .unwrap();

        assert!(data.get_with_expiration("src").unwrap().0.is_none());
        assert_eq!(builder.serialize(), "*1\r\n:1\r\n");
    }

    #[test]
    fn test_persist_fails() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        data.set("src", Value::String("value".to_string())).unwrap();

        run(
            data.clone(),
            vec![ProtocolType::String("src".to_string())],
            &mut builder,
        )
        .unwrap();

        assert!(data.get_with_expiration("src").unwrap().0.is_none());
        assert_eq!(builder.serialize(), "*1\r\n:0\r\n");
    }
}
