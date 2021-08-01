use crate::storage::data_storage::DataStorage;
use redis_protocol::response::ResponseBuilder;
use redis_protocol::types::ProtocolType;
use std::sync::Arc;

/// Insert all the specified values at the tail of the list stored at key. If key does not exist, it is created as empty list before performing the push operation.
pub fn run(
    builder: &mut ResponseBuilder,
    arguments: Vec<ProtocolType>,
    data: Arc<DataStorage>,
) -> Result<(), &'static str> {
    if arguments.len() < 2 {
        return Err("rpush must have arguments");
    }

    let string_arguments: Vec<String> = arguments
        .into_iter()
        .map(|x| x.string())
        .collect::<Result<_, _>>()?;

    let key = string_arguments[0].clone();
    match data.rpush(key, string_arguments[1..].to_owned()) {
        Ok(len) => {
            builder.add(ProtocolType::Integer(len as i64));
            Ok(())
        }
        Err(s) => Err(s),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::data_storage::DataStorage;
    use crate::storage::data_storage::Value;
    use redis_protocol::types::ProtocolType;
    use std::sync::Arc;

    #[test]
    fn insert_one_key() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        data.set("Test", Value::Vec(["value".to_string()].to_vec()))
            .unwrap();

        run(
            &mut builder,
            vec![
                ProtocolType::String("Test".to_string()),
                ProtocolType::String("value2".to_string()),
            ],
            data.clone(),
        )
        .unwrap();

        assert_eq!(
            vec!["value", "value2"],
            data.get("Test").unwrap().array().unwrap()
        );
        assert_eq!(":2\r\n", builder.serialize());
    }

    #[test]
    fn insert_keys() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        data.set("Test", Value::Vec(["1".to_string()].to_vec()))
            .unwrap();

        run(
            &mut builder,
            vec![
                ProtocolType::String("Test".to_string()),
                ProtocolType::String("2".to_string()),
                ProtocolType::String("3".to_string()),
                ProtocolType::String("4".to_string()),
            ],
            data.clone(),
        )
        .unwrap();

        assert_eq!(
            vec!["1", "2", "3", "4"],
            data.get("Test").unwrap().array().unwrap()
        );
        assert_eq!(":4\r\n", builder.serialize());
    }

    #[test]
    fn insert_to_a_not_existing_key() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();

        run(
            &mut builder,
            vec![
                ProtocolType::String("Test".to_string()),
                ProtocolType::String("1".to_string()),
                ProtocolType::String("2".to_string()),
            ],
            data.clone(),
        )
        .unwrap();

        assert_eq!(vec!["1", "2"], data.get("Test").unwrap().array().unwrap());
        assert_eq!(":2\r\n", builder.serialize());
    }
}
