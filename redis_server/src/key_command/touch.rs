use crate::logging::logger::Logger;
use crate::storage::data_storage::DataStorage;
use redis_protocol::response::ResponseBuilder;
use redis_protocol::types::ProtocolType;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

/// Sets the last access of the keys given to the actual time
/// Returns the amount of keys touched
pub fn run(
    builder: &mut ResponseBuilder,
    arguments: Vec<ProtocolType>,
    data: &Arc<DataStorage>,
    logger: Arc<Logger>,
) -> Result<(), &'static str> {
    let mut keys_touched = 0;
    let now_res = SystemTime::now().duration_since(UNIX_EPOCH);

    if now_res.is_err() {
        return Err("An error ocurred while getting the actual timestamp.");
    }

    let now = now_res.unwrap();

    for key in arguments.iter() {
        let str_key = key.clone().string()?;
        if let Ok(last_access) = data.modify_last_key_access(&str_key, now) {
            let _res = logger.log(&format!(
                "Previous last access from touch command: {}",
                last_access.as_secs()
            ));
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
    use std::fs;

    #[test]
    fn test_touch_one_key() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        let logger = Arc::new(Logger::new(".TEST.txt").unwrap());

        data.set("src", Value::String("value".to_string())).unwrap();

        run(
            &mut builder,
            vec![ProtocolType::String("src".to_string())],
            &data.clone(),
            logger,
        )
        .unwrap();
        let _ = fs::remove_file(".TEST.txt");
        assert_eq!(builder.serialize(), ":1\r\n");
    }

    #[test]
    fn test_touch_two_key() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        let logger = Arc::new(Logger::new(".TEST.txt").unwrap());

        data.set("src", Value::String("value".to_string())).unwrap();
        data.set("asd", Value::String("value".to_string())).unwrap();

        run(
            &mut builder,
            vec![
                ProtocolType::String("src".to_string()),
                ProtocolType::String("asd".to_string()),
            ],
            &data.clone(),
            logger,
        )
        .unwrap();

        let _ = fs::remove_file(".TEST.txt");
        assert_eq!(builder.serialize(), ":2\r\n");
    }

    #[test]
    fn test_send_two_but_only_one_touched() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        let logger = Arc::new(Logger::new(".TEST.txt").unwrap());

        data.set("src", Value::String("value".to_string())).unwrap();

        run(
            &mut builder,
            vec![
                ProtocolType::String("src".to_string()),
                ProtocolType::String("asd".to_string()),
            ],
            &data.clone(),
            logger,
        )
        .unwrap();
        let _ = fs::remove_file(".TEST.txt");
        assert_eq!(builder.serialize(), ":1\r\n");
    }

    #[test]
    fn test_no_keys_touched() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        let logger = Arc::new(Logger::new(".TEST.txt").unwrap());

        run(
            &mut builder,
            vec![
                ProtocolType::String("src".to_string()),
                ProtocolType::String("asd".to_string()),
            ],
            &data.clone(),
            logger,
        )
        .unwrap();

        let _ = fs::remove_file(".TEST.txt");
        assert_eq!(builder.serialize(), ":0\r\n");
    }
}
