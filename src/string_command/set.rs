use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use crate::storage::data_storage::{DataStorage, Value};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn run(
    db: Arc<DataStorage>,
    arguments: Vec<ProtocolType>,
    builder: &mut ResponseBuilder,
) -> Result<(), &'static str> {
    assert!(arguments.len() > 1);
    let name = arguments[0].clone().string()?;
    let value = Value::String(arguments[1].clone().string()?);
    let mut xx = false;
    let mut nx = false;
    let mut keepttl = false;
    let mut maybe_exp = None;
    let mut get = false;

    for i in 2..arguments.len() {
        let str = arguments[i].clone().string()?;
        let now = SystemTime::now().duration_since(UNIX_EPOCH).ok().ok_or("Cannot cast time")?;
        match &str.to_ascii_uppercase()[..] {
            "EX" => maybe_exp = Some(Duration::from_secs(arguments[i+1].clone().integer()? as u64) + now),
            "PX" => maybe_exp = Some(Duration::from_millis(arguments[i+1].clone().integer()? as u64) + now),
            "EXAT" => maybe_exp = Some(Duration::from_secs(arguments[i+1].clone().integer()? as u64)),
            "PXAT" => maybe_exp = Some(Duration::from_millis(arguments[i+1].clone().integer()? as u64)),
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
        } else {
            builder.add(ProtocolType::SimpleString("OK".to_string()));
        }
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

        assert_eq!(data.get("key1").unwrap().string().unwrap(), "Hello World");
        assert_eq!(builder.serialize(), "*1\r\n+OK\r\n");
    }

    #[test]
    fn test_empty_set() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();

        run(data.clone(), vec![], &mut builder).unwrap();

        assert_eq!(builder.serialize(), "*1\r\n+OK\r\n");
    }
}
