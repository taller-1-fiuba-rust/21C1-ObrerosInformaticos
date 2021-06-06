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
    let mut keys_touched = 0;
    let now_res = SystemTime::now().duration_since(UNIX_EPOCH);

    if let Err(_) = now_res {
        return Err("An error ocurred while getting the actual timestamp.");
    }

    let now = now_res.unwrap();

    for key in arguments.iter() {
        let str_key = key.clone().string()?;
        if let Ok(_) = data.modify_last_key_access(&str_key, now) {
            keys_touched += 1;
        }
    }
    builder.add(ProtocolType::Integer(keys_touched));
    Ok(())
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::storage::data_storage::Value;

    #[test]
    fn test_touch_one_key() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();

        data.add_key_value("src", Value::String("value".to_string()))
            .unwrap();

        run(
            &mut builder,
            vec![ProtocolType::String("src".to_string())],
            &data.clone(),
        )
        .unwrap();

        assert_eq!(builder.serialize(), "*1\r\n:1\r\n");
    }

    #[test]
    fn test_touch_two_key() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();

        data.add_key_value("src", Value::String("value".to_string()))
            .unwrap();
        data.add_key_value("asd", Value::String("value".to_string()))
            .unwrap();

        run(
            &mut builder,
            vec![
                ProtocolType::String("src".to_string()),
                ProtocolType::String("asd".to_string()),
            ],
            &data.clone(),
        )
        .unwrap();

        assert_eq!(builder.serialize(), "*1\r\n:2\r\n");
    }

    #[test]
    fn test_send_two_but_only_one_touched() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();

        data.add_key_value("src", Value::String("value".to_string()))
            .unwrap();

        run(
            &mut builder,
            vec![
                ProtocolType::String("src".to_string()),
                ProtocolType::String("asd".to_string()),
            ],
            &data.clone(),
        )
        .unwrap();

        assert_eq!(builder.serialize(), "*1\r\n:1\r\n");
    }

    #[test]
    fn test_no_keys_touched() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();

        run(
            &mut builder,
            vec![
                ProtocolType::String("src".to_string()),
                ProtocolType::String("asd".to_string()),
            ],
            &data.clone(),
        )
        .unwrap();

        assert_eq!(builder.serialize(), "*1\r\n:0\r\n");
    }
}
