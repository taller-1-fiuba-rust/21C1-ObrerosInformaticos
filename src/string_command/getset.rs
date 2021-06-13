use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use crate::storage::data_storage::DataStorage;
use crate::storage::data_storage::Value;
use std::sync::Arc;

pub fn run(
    builder: &mut ResponseBuilder,
    arguments: Vec<ProtocolType>,
    data: &Arc<DataStorage>,
) -> Result<(), &'static str> {
    if arguments.len() != 2 {
        return Err("Wrong quantity of arguments");
    }

    let key = arguments[0].clone().string()?;

    match data.get(&key) {
        Some(value) => match value {
            Value::String(old_value) => {
                let new_value = arguments[1].clone().string()?;
                data.set(&key, Value::String(new_value))?;
                builder.add(ProtocolType::String(old_value));
            }
            Value::Vec(_) => {
                return Err("WRONGTYPE Operation against a key holding the wrong kind of value")
            }
            Value::HashSet(_) => {
                return Err("WRONGTYPE Operation against a key holding the wrong kind of value")
            }
        },
        None => builder.add(ProtocolType::String("nil".to_string())),
    }
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

        assert_eq!(builder.serialize(), "*1\r\n$5\r\nvalue\r\n");
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

        assert_eq!(builder.serialize(), "*1\r\n$3\r\nnil\r\n");
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
