use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use crate::storage::data_storage::DataStorage;
use crate::storage::data_storage::Value;
use std::sync::Arc;

pub fn run(
    arguments: Vec<ProtocolType>,
    builder: &mut ResponseBuilder,
    data: &Arc<DataStorage>,
) -> Result<(), &'static str> {
    //This command
    if arguments.len() != 1 {
        return Err("Wrong quantity of arguments.");
    }

    let key = arguments[0].clone().string()?;

    let value_option = data.get(&key);

    match value_option {
        Some(Value::String(_)) => builder.add(ProtocolType::SimpleString("string".to_string())),
        Some(Value::Vec(_)) => builder.add(ProtocolType::SimpleString("vec".to_string())),
        Some(Value::HashSet(_)) => builder.add(ProtocolType::SimpleString("set".to_string())),
        None => builder.add(ProtocolType::SimpleString("none".to_string())),
    }
    Ok(())
}

#[cfg(test)]

mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_type_string() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();

        data.set("src", Value::String("value".to_string())).unwrap();

        run(
            vec![ProtocolType::String("src".to_string())],
            &mut builder,
            &data.clone(),
        )
        .unwrap();

        assert_eq!(builder.serialize(), "+string\r\n");
    }

    #[test]
    fn test_type_vec() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();

        data.set("src", Value::String("value".to_string())).unwrap();

        run(
            vec![ProtocolType::String("src".to_string())],
            &mut builder,
            &data.clone(),
        )
        .unwrap();

        assert_eq!(builder.serialize(), "+string\r\n");
    }

    #[test]
    fn test_type_set() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();

        data.set("src", Value::HashSet(HashSet::new())).unwrap();

        run(
            vec![ProtocolType::String("src".to_string())],
            &mut builder,
            &data.clone(),
        )
        .unwrap();

        assert_eq!(builder.serialize(), "+set\r\n");
    }

    #[test]
    fn test_no_key() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        run(
            vec![ProtocolType::String("src".to_string())],
            &mut builder,
            &data.clone(),
        )
        .unwrap();
        assert_eq!(builder.serialize(), "+none\r\n");
    }
}
