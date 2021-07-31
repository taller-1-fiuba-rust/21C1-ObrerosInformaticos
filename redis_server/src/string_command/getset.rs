use redis_protocol::response::ResponseBuilder;
use redis_protocol::types::ProtocolType;
use crate::storage::data_storage::DataStorage;
use crate::storage::data_storage::Value;
use std::sync::Arc;

/// Atomically sets key to value and returns the old value stored at key.
/// Returns an error when key exists but does not hold a string value.
/// Any previous time to live associated with the key is discarded on successful SET operation.
pub fn run(
    builder: &mut ResponseBuilder,
    arguments: Vec<ProtocolType>,
    data: &Arc<DataStorage>,
) -> Result<(), &'static str> {
    if arguments.len() != 2 {
        return Err("Wrong quantity of arguments");
    }

    let key = arguments[0].clone().string()?;
    let new_value = arguments[1].clone().string()?;

    let response = data.getset(&key, Value::String(new_value))?;
    builder.add(ProtocolType::String(response));
    Ok(())
}

#[cfg(test)]

mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_getset_ok() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();

        data.set("src", Value::String("value".to_string())).unwrap();

        run(
            &mut builder,
            vec![
                ProtocolType::String("src".to_string()),
                ProtocolType::String("new_value".to_string()),
            ],
            &data.clone(),
        )
        .unwrap();

        assert_eq!(builder.serialize(), "$5\r\nvalue\r\n");
        assert_eq!(data.get("src").unwrap().string().unwrap(), "new_value");
    }

    #[test]
    fn test_getset_nil() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();

        run(
            &mut builder,
            vec![
                ProtocolType::String("src".to_string()),
                ProtocolType::String("new_value".to_string()),
            ],
            &data.clone(),
        )
        .unwrap();

        assert_eq!(builder.serialize(), "$3\r\nnil\r\n");
    }

    #[test]
    fn test_getset_err_list() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();

        data.set("src", Value::Vec(vec!["value".to_string()]))
            .unwrap();

        let result = run(
            &mut builder,
            vec![
                ProtocolType::String("src".to_string()),
                ProtocolType::String("new_value".to_string()),
            ],
            &data.clone(),
        );

        match result {
            Ok(_) => assert_eq!(true, false),
            Err(msg) => assert_eq!(
                msg,
                "WRONGTYPE Operation against a key holding the wrong kind of value"
            ),
        }
    }

    #[test]
    fn test_getset_err_set() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();

        data.set("src", Value::HashSet(HashSet::new())).unwrap();

        let result = run(
            &mut builder,
            vec![
                ProtocolType::String("src".to_string()),
                ProtocolType::String("new_value".to_string()),
            ],
            &data.clone(),
        );

        match result {
            Ok(_) => assert_eq!(true, false),
            Err(msg) => assert_eq!(
                msg,
                "WRONGTYPE Operation against a key holding the wrong kind of value"
            ),
        }
    }
}
