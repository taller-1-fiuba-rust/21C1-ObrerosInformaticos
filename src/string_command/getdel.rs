use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use crate::storage::data_storage::DataStorage;
use std::sync::Arc;

pub fn run(
    arguments: Vec<ProtocolType>,
    builder: &mut ResponseBuilder,
    data: Arc<DataStorage>,
) -> Result<(), &'static str> {
    if arguments.len() != 1 {
        return Err("ERR wrong number of arguments");
    }

    let key = arguments[0].clone().string()?;
    let value = data.get_string_value(key);

    match value {
        Ok(s) => match s {
            Some(value) => {
                let key = arguments[0].clone().string()?;
                data.delete_key(&key)?;
                builder.add(ProtocolType::String(value));
                Ok(())
            }
            None => {
                builder.add(ProtocolType::String("(nil)".to_string()));
                Ok(())
            }
        },
        Err(_i) => Err("Value not a string"),
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::storage::data_storage::Value;

    #[test]
    fn test_getdel_existing_key() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        data.set("key", Value::String("value".to_string())).unwrap();

        run(
            vec![ProtocolType::String("key".to_string())],
            &mut builder,
            data.clone(),
        )
        .unwrap();

        assert_eq!(builder.serialize(), "$5\r\nvalue\r\n");

        assert_eq!(data.exists_key("key"), Err("Not key in HashMap"));
    }

    #[test]
    fn test_getdel_not_existing_key() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();

        run(
            vec![ProtocolType::String("key".to_string())],
            &mut builder,
            data.clone(),
        )
        .unwrap();

        assert_eq!(builder.serialize(), "$5\r\n(nil)\r\n");
    }
}
