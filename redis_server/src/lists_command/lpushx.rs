use crate::storage::data_storage::DataStorage;
use redis_protocol::response::ResponseBuilder;
use redis_protocol::types::ProtocolType;
use std::sync::Arc;

/// Inserts specified values at the head of the list stored at key,
/// only if key already exists and holds a list.
/// In contrary to LPUSH, no operation will be performed when key does not yet exist.
pub fn run(
    builder: &mut ResponseBuilder,
    arguments: Vec<ProtocolType>,
    data: Arc<DataStorage>,
) -> Result<(), &'static str> {
    if arguments.len() < 2 {
        return Err("lpushx must have arguments");
    }

    let mut string_arguments: Vec<String> = arguments
        .into_iter()
        .map(|x| x.string())
        .collect::<Result<_, _>>()?;

    let key = string_arguments[0].clone();
    string_arguments.remove(0);

    let list_len = data.lpushx(key, string_arguments);

    match list_len {
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

        assert_eq!(":4\r\n", builder.serialize());
    }

    #[test]
    fn insert_to_a_not_existing_key() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();

        run(
            &mut builder,
            vec![
                ProtocolType::String("1".to_string()),
                ProtocolType::String("2".to_string()),
            ],
            data.clone(),
        )
        .unwrap();

        assert_eq!(":0\r\n", builder.serialize());
    }
}
