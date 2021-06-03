use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use crate::storage::data_storage::DataStorage;
use std::sync::Arc;

pub fn run(
    builder: &mut ResponseBuilder,
    arguments: Vec<ProtocolType>,
    data: &Arc<DataStorage>,
) -> Result<(), &'static str> {
    let mut string_arguments = vec![];

    for argument in arguments {
        string_arguments.push(argument.clone().string()?);
    }

    let mut counter = 0;

    for arg in string_arguments {
        let result = data.delete_key(&arg);

        if result == Ok(()) {
            counter += 1;
        }
    }

    builder.add(ProtocolType::Integer(counter));
    Ok(())
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::protocol::types::ProtocolType;
    use crate::storage::data_storage::DataStorage;
    use crate::storage::data_storage::Value;
    use std::sync::Arc;

    #[test]
    fn delete_one_key() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        data.add_key_value("Test", Value::String("value".to_string()));

        run(
            &mut builder,
            vec![ProtocolType::String("Test".to_string())],
            &data.clone(),
        )
        .unwrap();

        assert_eq!("*1\r\n:1\r\n", builder.serialize());
    }

    #[test]
    fn delete_keys() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        data.add_key_value("1", Value::String("value".to_string()));
        data.add_key_value("2", Value::String("value".to_string()));
        data.add_key_value("3", Value::String("value".to_string()));
        data.add_key_value("4", Value::String("value".to_string()));

        run(
            &mut builder,
            vec![
                ProtocolType::String("1".to_string()),
                ProtocolType::String("2".to_string()),
                ProtocolType::String("3".to_string()),
                ProtocolType::String("4".to_string()),
            ],
            &data.clone(),
        )
        .unwrap();

        assert_eq!("*1\r\n:4\r\n", builder.serialize());
    }

    #[test]
    fn delete_not_existing_keys() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();

        run(
            &mut builder,
            vec![
                ProtocolType::String("1".to_string()),
                ProtocolType::String("2".to_string()),
                ProtocolType::String("3".to_string()),
                ProtocolType::String("4".to_string()),
            ],
            &data.clone(),
        )
        .unwrap();

        assert_eq!("*1\r\n:0\r\n", builder.serialize());
    }
}
