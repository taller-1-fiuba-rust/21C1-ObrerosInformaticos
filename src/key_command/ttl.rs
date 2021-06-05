use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use crate::storage::data_storage::DataStorage;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn run(
    builder: &mut ResponseBuilder,
    arguments: Vec<ProtocolType>,
    data: &Arc<DataStorage>,
) -> Result<(), &'static str> {
    if arguments.len() != 1 {
        return Err("Wrong quantity of arguments. Command TTL has only one.");
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

        assert_eq!(builder.serialize(), "*1\r\n:100\r\n");
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

        assert_eq!(builder.serialize(), "*1\r\n:-2\r\n");
    }

    #[test]
    fn test_ttl_with_no_expiration() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();

        data.add_key_value("asd", Value::String("value".to_string()));

        run(
            &mut builder,
            vec![ProtocolType::String("asd".to_string())],
            &data.clone(),
        )
        .unwrap();

        assert_eq!(builder.serialize(), "*1\r\n:-1\r\n");
    }
}
