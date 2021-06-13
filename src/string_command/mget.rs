use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use crate::storage::data_storage::DataStorage;
use crate::storage::data_storage::Value;
use std::sync::Arc;

pub fn run(
    arguments: Vec<ProtocolType>,
    builder: &mut ResponseBuilder,
    data: Arc<DataStorage>,
) -> Result<(), &'static str> {
    let mut response = Vec::new();

    for key in arguments.iter() {
        let string_key = key.clone().string()?;
        let result = data.get(&string_key);
        match result {
            Some(value) => match value {
                Value::String(string) => response.push(ProtocolType::String(string)),
                Value::Vec(_) => response.push(ProtocolType::String("nil".to_string())),
                Value::HashSet(_) => response.push(ProtocolType::String("nil".to_string())),
            },
            None => response.push(ProtocolType::String("nil".to_string())),
        }
    }

    builder.add(ProtocolType::Array(response));
    Ok(())
}

#[cfg(test)]

mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_gets_key1_and_2_nils() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();

        data.set("key1", Value::String("value".to_string()))
            .unwrap();
        data.set("key2", Value::Vec(vec![])).unwrap();

        run(
            vec![
                ProtocolType::String("key1".to_string()),
                ProtocolType::String("key2".to_string()),
                ProtocolType::String("XX".to_string()),
            ],
            &mut builder,
            data,
        )
        .unwrap();

        assert_eq!(
            builder.serialize(),
            "*1\r\n*3\r\n$5\r\nvalue\r\n$3\r\nnil\r\n$3\r\nnil\r\n"
        );
    }
}
