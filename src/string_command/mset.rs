use crate::protocol::types::ProtocolType;
use crate::protocol::response::ResponseBuilder;
use crate::storage::data_storage::{DataStorage, Value};
use std::sync::Arc;

pub fn run(
    db: Arc<DataStorage>,
    arguments: Vec<ProtocolType>,
    builder: &mut ResponseBuilder,
) -> Result<(), &'static str> {
    let mut names = vec![];
    let mut values = vec![];
    let mut i = 0;
    for argument in arguments {
        let str = argument.clone().string()?;
        if i % 2 == 0 {
            names.push(str);
        } else {
            values.push(Value::String(str));
        }
        i += 1;
    }

    db.set_multiple(names, values)?;

    builder.add(ProtocolType::SimpleString("OK".to_string()));
    Ok(())
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_mset() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();

        run(
            data.clone(),
            vec![
                ProtocolType::String("key1".to_string()),
                ProtocolType::String("Hello".to_string()),
                ProtocolType::String("key2".to_string()),
                ProtocolType::String("World".to_string()),
            ],
            &mut builder,
        ).unwrap();

        assert_eq!(data.get("key1").unwrap().string().unwrap(), "Hello");
        assert_eq!(data.get("key2").unwrap().string().unwrap(), "World");
        assert_eq!(builder.serialize(), "*1\r\n+OK\r\n");
    }

    #[test]
    fn test_empty_mset() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();

        run(
            data.clone(),
            vec![],
            &mut builder,
        )
            .unwrap();

        assert_eq!(builder.serialize(), "*1\r\n+OK\r\n");
    }
}
