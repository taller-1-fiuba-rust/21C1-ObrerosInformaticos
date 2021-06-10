use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use crate::storage::data_storage::{DataStorage, Value};
use std::sync::Arc;

pub fn run(
    db: Arc<DataStorage>,
    arguments: Vec<ProtocolType>,
    builder: &mut ResponseBuilder,
) -> Result<(), &'static str> {
    assert!(arguments.len() > 1);
    let name = arguments[0].string()?;

    db.set(names, values);

    builder.add(ProtocolType::SimpleString("OK".to_string()));
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
