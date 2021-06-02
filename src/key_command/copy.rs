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

    let option = db.get(&src);
    let mut result = 0;
    if let Some(value) = option {
        db.add_key_value(&dst, value.clone());
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
        assert_eq!(builder.serialize(), "*1\r\n:1\r\n");
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

        assert_eq!(builder.serialize(), "*1\r\n:0\r\n");
    }
}
