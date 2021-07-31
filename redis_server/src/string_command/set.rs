use redis_protocol::response::ResponseBuilder;
use redis_protocol::types::ProtocolType;
use crate::storage::data_storage::{DataStorage, Value};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Set key to hold the string value. If key already holds a value, it is overwritten, regardless of its type.
/// Any previous time to live associated with the key is discarded on successful SET operation.
pub fn run(
    db: Arc<DataStorage>,
    arguments: Vec<ProtocolType>,
    builder: &mut ResponseBuilder,
) -> Result<(), &'static str> {
    if arguments.len() <= 1 {
        return Err("Wrong number of arguments");
    }

    let name = arguments[0].clone().string()?;
    let value = Value::String(arguments[1].clone().string()?);
    let mut xx = false;
    let mut nx = false;
    let mut keepttl = false;
    let mut maybe_exp = None;
    let mut get = false;

    for i in 2..arguments.len() {
        let str = arguments[i].clone().string()?;
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .ok()
            .ok_or("Cannot cast time")?;
        match &str.to_ascii_uppercase()[..] {
            "EX" => {
                maybe_exp =
                    Some(Duration::from_secs(arguments[i + 1].clone().integer()? as u64) + now)
            }
            "PX" => {
                maybe_exp =
                    Some(Duration::from_millis(arguments[i + 1].clone().integer()? as u64) + now)
            }
            "EXAT" => {
                maybe_exp = Some(Duration::from_secs(
                    arguments[i + 1].clone().integer()? as u64
                ))
            }
            "PXAT" => {
                maybe_exp = Some(Duration::from_millis(
                    arguments[i + 1].clone().integer()? as u64
                ))
            }
            "NX" => nx = true,
            "XX" => xx = true,
            "KEEPTTL" => keepttl = true,
            "GET" => get = true,
            _ => {}
        }
    }

    let old = db.get_with_expiration(&name);
    if old.is_none() && nx || old.is_some() && xx || !nx && !xx {
        db.set(&name, value)?;
        if let Some((old_exp, v)) = old {
            if let Some(exp) = maybe_exp {
                db.set_expiration_to_key(Some(exp), &name)?;
            }

            if keepttl {
                db.set_expiration_to_key(old_exp, &name)?;
            }

            if get {
                builder.add(ProtocolType::String(v.string()?));
            }
        }
    }

    if builder.is_empty() {
        builder.add(ProtocolType::SimpleString("OK".to_string()));
    }

    Ok(())
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_set() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();

        run(
            data.clone(),
            vec![
                ProtocolType::String("key1".to_string()),
                ProtocolType::String("Hello World".to_string()),
            ],
            &mut builder,
        )
        .unwrap();

        assert!(data.get_with_expiration("key1").unwrap().0.is_none());
        assert_eq!(data.get("key1").unwrap().string().unwrap(), "Hello World");
        assert_eq!(builder.serialize(), "+OK\r\n");
    }

    #[test]
    fn test_set_xx() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        data.set("key1", Value::String("previous".to_string()))
            .unwrap();

        run(
            data.clone(),
            vec![
                ProtocolType::String("key1".to_string()),
                ProtocolType::String("Hello World".to_string()),
                ProtocolType::String("XX".to_string()),
            ],
            &mut builder,
        )
        .unwrap();

        assert_eq!(data.get("key1").unwrap().string().unwrap(), "Hello World");
        assert_eq!(builder.serialize(), "+OK\r\n");
    }

    #[test]
    fn test_set_nx() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        data.set("key1", Value::String("previous".to_string()))
            .unwrap();

        run(
            data.clone(),
            vec![
                ProtocolType::String("key1".to_string()),
                ProtocolType::String("Hello World".to_string()),
                ProtocolType::String("NX".to_string()),
            ],
            &mut builder,
        )
        .unwrap();

        assert_eq!(data.get("key1").unwrap().string().unwrap(), "previous");
        assert_eq!(builder.serialize(), "+OK\r\n");
    }

    #[test]
    fn test_set_get() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        data.set("key1", Value::String("PREV".to_string())).unwrap();

        run(
            data.clone(),
            vec![
                ProtocolType::String("key1".to_string()),
                ProtocolType::String("Hello World".to_string()),
                ProtocolType::String("GET".to_string()),
            ],
            &mut builder,
        )
        .unwrap();

        assert_eq!(data.get("key1").unwrap().string().unwrap(), "Hello World");
        assert_eq!(builder.serialize(), "$4\r\nPREV\r\n");
    }

    #[test]
    fn test_set_ex() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        data.set("key1", Value::String("PREV".to_string())).unwrap();

        run(
            data.clone(),
            vec![
                ProtocolType::String("key1".to_string()),
                ProtocolType::String("Hello World".to_string()),
                ProtocolType::String("EX".to_string()),
                ProtocolType::String("60".to_string()),
            ],
            &mut builder,
        )
        .unwrap();

        assert!(data.get_with_expiration("key1").unwrap().0.is_some());
        assert_eq!(data.get("key1").unwrap().string().unwrap(), "Hello World");
        assert_eq!(builder.serialize(), "+OK\r\n");
    }
}
