use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use crate::storage::data_storage::DataStorage;
use std::sync::Arc;

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
    use crate::protocol::types::ProtocolType;
    use crate::storage::data_storage::DataStorage;
    use crate::storage::data_storage::Value;
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
