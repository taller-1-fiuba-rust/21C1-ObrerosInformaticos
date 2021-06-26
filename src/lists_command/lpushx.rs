use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use crate::storage::data_storage::DataStorage;
use std::sync::Arc;

pub fn run(
    builder: &mut ResponseBuilder,
    arguments: Vec<ProtocolType>,
    data: Arc<DataStorage>,
) -> Result<(), &'static str> {
    if arguments.is_empty() {
        return Err("lpushx must have arguments");
    }

    let mut string_arguments: Vec<String> = arguments
        .into_iter()
        .map(|x| x.string())
        .into_iter()
        .collect::<Result<_, _>>()?;

    let key = string_arguments[0].clone();
    string_arguments.remove(0);

    let list_len = data.lpushx(key, string_arguments);

    match list_len {
        Ok(len) => {
            builder.add(ProtocolType::Integer(len as i64));
            Ok(())
        }
        Err(s) => {
            builder.add(ProtocolType::Error("lpushx not executed".to_string()));
            Err(s)
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::protocol::types::ProtocolType;
    use crate::storage::data_storage::DataStorage;
    use crate::storage::data_storage::Value;
    use std::sync::Arc;

    #[test]
    fn insert_one_key() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        data.set("Test", Value::Vec(["value".to_string()].to_vec()))
            .unwrap();

        run(
            &mut builder,
            vec![
                ProtocolType::String("Test".to_string()),
                ProtocolType::String("value2".to_string()),
            ],
            data.clone(),
        )
        .unwrap();

        assert_eq!(":2\r\n", builder.serialize());
    }

    #[test]
    fn insert_keys() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        data.set("Test", Value::Vec(["1".to_string()].to_vec()))
            .unwrap();

        run(
            &mut builder,
            vec![
                ProtocolType::String("Test".to_string()),
                ProtocolType::String("2".to_string()),
                ProtocolType::String("3".to_string()),
                ProtocolType::String("4".to_string()),
            ],
            data.clone(),
        )
        .unwrap();

        assert_eq!(":4\r\n", builder.serialize());
    }

    #[test]
    fn insert_to_a_not_existing_key() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();

        run(
            &mut builder,
            vec![
                ProtocolType::String("1".to_string()),
                ProtocolType::String("2".to_string()),
            ],
            data.clone(),
        )
        .unwrap();

        assert_eq!(":0\r\n", builder.serialize());
    }
}
