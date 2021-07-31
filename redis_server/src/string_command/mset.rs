use redis_protocol::response::ResponseBuilder;
use redis_protocol::types::ProtocolType;
use crate::storage::data_storage::{DataStorage, Value};
use std::sync::Arc;

/// Sets the given keys to their respective values. MSET replaces existing values with new values, just as regular SET.
pub fn run(
    db: Arc<DataStorage>,
    arguments: Vec<ProtocolType>,
    builder: &mut ResponseBuilder,
) -> Result<(), &'static str> {
    let mut names = vec![];
    let mut values = vec![];

    for (i, argument) in arguments.into_iter().enumerate() {
        let str = argument.string()?;
        if i % 2 == 0 {
            names.push(str);
        } else {
            values.push(Value::String(str));
        }
    }

    db.set_multiple(names, values)?;

    builder.add(ProtocolType::SimpleString("OK".to_string()));
    Ok(())
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_mset() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();

        run(
            data.clone(),
            vec![
                ProtocolType::String("key1".to_string()),
                ProtocolType::String("Hello".to_string()),
                ProtocolType::String("key2".to_string()),
                ProtocolType::String("World".to_string()),
            ],
            &mut builder,
        )
        .unwrap();

        assert_eq!(data.get("key1").unwrap().string().unwrap(), "Hello");
        assert_eq!(data.get("key2").unwrap().string().unwrap(), "World");
        assert_eq!(builder.serialize(), "+OK\r\n");
    }

    #[test]
    fn test_empty_mset() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();

        run(data.clone(), vec![], &mut builder).unwrap();

        assert_eq!(builder.serialize(), "+OK\r\n");
    }
}
