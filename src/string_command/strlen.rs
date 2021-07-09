use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use crate::storage::data_storage::{DataStorage, Value};
use std::sync::Arc;

pub fn run(
    db: Arc<DataStorage>,
    arguments: Vec<ProtocolType>,
    builder: &mut ResponseBuilder,
) -> Result<(), &'static str> {
    if arguments.len() != 1 {
        return Err("Wrong number of arguments");
    }

    let key = arguments[0].clone().string()?;
    let maybe_val = db.get(&key);
    if let Some(val) = maybe_val {
        match val {
            Value::String(s) => builder.add(ProtocolType::Integer(s.len() as i64)),
            Value::HashSet(_) => return Err("Stored value is a hashset"),
            Value::Vec(_) => return Err("Stored value is a list"),
        }
    } else {
        builder.add(ProtocolType::Integer(0));
    }
    Ok(())
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_strlen() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        data.set("key", Value::String("value".to_string())).unwrap();

        run(
            data.clone(),
            vec![ProtocolType::String("key".to_string())],
            &mut builder,
        )
        .unwrap();

        assert_eq!(builder.serialize(), ":5\r\n");
    }

    #[test]
    #[should_panic]
    fn test_non_string() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();

        run(data.clone(), vec![], &mut builder).unwrap();
    }

    #[test]
    fn test_inexistant() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();

        run(
            data.clone(),
            vec![ProtocolType::String("no_such_key".to_string())],
            &mut builder,
        )
        .unwrap();

        assert_eq!(builder.serialize(), ":0\r\n");
    }
}
