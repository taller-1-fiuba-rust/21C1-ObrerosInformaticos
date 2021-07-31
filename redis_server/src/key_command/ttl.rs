use redis_protocol::response::ResponseBuilder;
use redis_protocol::types::ProtocolType;
use crate::storage::data_storage::DataStorage;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

/// Returns the remaining time to live of a key that has a timeout.
/// -1 if there's no expiration for the given key
/// -2 if theres no value for that key
pub fn run(
    builder: &mut ResponseBuilder,
    arguments: Vec<ProtocolType>,
    data: &Arc<DataStorage>,
) -> Result<(), &'static str> {
    if arguments.len() != 1 {
        return Err("ERR wrong number of arguments for 'ttl' command");
    }

    let key = arguments[0].clone().string()?;
    let value_option = data.get_with_expiration(&key);
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    match value_option {
        Some((duration_op, _)) => match duration_op {
            Some(duration) => {
                let expiration = duration.as_secs() as i64;

                builder.add(ProtocolType::Integer(expiration - now as i64));
            }
            None => builder.add(ProtocolType::Integer(-1)),
        },
        None => builder.add(ProtocolType::Integer(-2)),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::data_storage::Value;
    use std::time::Duration;

    #[test]
    fn test_key_with_expiration_ok() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();

        let expiration_time = SystemTime::now()
            .checked_add(Duration::from_secs(100))
            .unwrap()
            .duration_since(UNIX_EPOCH)
            .unwrap();

        data.add_with_expiration("src", Value::String("value".to_string()), expiration_time)
            .unwrap();

        run(
            &mut builder,
            vec![ProtocolType::String("src".to_string())],
            &data.clone(),
        )
        .unwrap();

        assert_eq!(builder.serialize(), ":100\r\n");
    }

    #[test]
    fn test_tll_with_wrong_key() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        run(
            &mut builder,
            vec![ProtocolType::String("src".to_string())],
            &data.clone(),
        )
        .unwrap();

        assert_eq!(builder.serialize(), ":-2\r\n");
    }

    #[test]
    fn test_ttl_with_no_expiration() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();

        data.set("asd", Value::String("value".to_string())).unwrap();

        run(
            &mut builder,
            vec![ProtocolType::String("asd".to_string())],
            &data.clone(),
        )
        .unwrap();

        assert_eq!(builder.serialize(), ":-1\r\n");
    }
}
