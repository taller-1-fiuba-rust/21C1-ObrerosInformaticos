use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use crate::storage::data_storage::DataStorage;
use std::sync::Arc;

pub fn run(
    arguments: Vec<ProtocolType>,
    builder: &mut ResponseBuilder,
    data: Arc<DataStorage>,
) -> Result<(), &'static str> {
    if arguments.len() != 2 {
        return Err("ERR wrong number of arguments for 'append' command");
    }

    let key = arguments[0].clone().string()?;
    let value = arguments[1].clone().string()?;

    let value_length = data.append(key, value);

    match value_length {
        Ok(s) => {
            builder.add(ProtocolType::Integer(s as i64));
            Ok(())
        }
        Err(_i) => Err("string not appended"),
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::storage::data_storage::Value;

    #[test]
    fn test_append_existing_key() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        data.set("key", Value::String("value".to_string())).unwrap();

        run(
            vec![
                ProtocolType::String("key".to_string()),
                ProtocolType::String("_append_value".to_string()),
            ],
            &mut builder,
            data.clone(),
        )
        .unwrap();

        assert_eq!(builder.serialize(), ":18\r\n");
        assert_eq!(
            data.get("key").unwrap().string().unwrap(),
            "value_append_value"
        );
    }

    #[test]
    fn test_not_append_existing_key() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();

        run(
            vec![
                ProtocolType::String("key".to_string()),
                ProtocolType::String("value".to_string()),
            ],
            &mut builder,
            data.clone(),
        )
        .unwrap();

        assert_eq!(data.get("key").unwrap().string().unwrap(), "value");
        assert_eq!(builder.serialize(), ":5\r\n");
    }
}
