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
        return Err("Wrong quantity of arguments. Command TYPE has only one.");
    }

    let key = arguments[0].clone().string()?;

    let value_option = data.get(&key);

    if let Some(value) = value_option {
        match value {
            Value::String(_) => builder.add(ProtocolType::SimpleString("string".to_string())),
            Value::Vec(_) => builder.add(ProtocolType::SimpleString("vec".to_string())),
            Value::HashSet(_) => builder.add(ProtocolType::SimpleString("hashset".to_string())),
        }
    } else {
        return Err("There's no value for that key");
    }
    Ok(())
}

mod tests {
    use super::*;
    use crate::storage::data_storage::Value;

    #[test]
    fn test_type_string() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();

        data.add_key_value("src", Value::String("value".to_string()));

        run(
            
            vec![ProtocolType::String("src".to_string())],
            &mut builder,
            &data.clone(),
        )
        .unwrap();

        assert_eq!(builder.serialize(), "*1\r\n+string\r\n");
    }
}