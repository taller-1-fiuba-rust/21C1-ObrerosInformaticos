use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use crate::storage::data_storage::DataStorage;
use std::sync::Arc;

pub fn run(
    builder: &mut ResponseBuilder,
    arguments: Vec<ProtocolType>,
    data: Arc<DataStorage>,
) -> Result<(), &'static str> {
    if arguments.len() < 2 {
        return Err("ERR wrong number of arguments for 'sadd' command");
    }
    let key = arguments[0].clone().string()?;

    let string_arguments: Vec<String> = arguments
        .into_iter()
        .map(|x| x.string())
        .collect::<Result<_, _>>()?;

    let result = data.sadd(key, string_arguments[1..].to_owned());

    match result {
        Ok(s) => {
            builder.add(ProtocolType::Integer(s));
            Ok(())
        }
        Err(s) => Err(s),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::types::ProtocolType;
    use crate::storage::data_storage::DataStorage;
    use crate::storage::data_storage::Value;
    use std::collections::HashSet;
    use std::sync::Arc;

    #[test]
    fn sadd_3_different_new_values() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        run(
            &mut builder,
            vec![
                ProtocolType::String("Test".to_string()),
                ProtocolType::String("1".to_string()),
                ProtocolType::String("2".to_string()),
                ProtocolType::String("3".to_string())
            ],
            data.clone(),
        )
        .unwrap();

        assert!(data.contains_key("Test".to_string()));
        assert_eq!(":3\r\n", builder.serialize());
    }

    #[test]
    fn sadd_3_different_to_already_setted_set() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        let set: HashSet<String> = HashSet::new();
        data.set("Test", Value::HashSet(set)).unwrap();
        run(
            &mut builder,
            vec![
                ProtocolType::String("Test".to_string()),
                ProtocolType::String("1".to_string()),
                ProtocolType::String("2".to_string()),
                ProtocolType::String("3".to_string())
            ],
            data.clone(),
        )
        .unwrap();

        assert!(data.contains_key("Test".to_string()));
        assert_eq!(":3\r\n", builder.serialize());
    }

    #[test]
    fn sadd_3_values_some_new_some_old() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        let mut set: HashSet<String> = HashSet::new();
        set.insert("1".to_string());
        set.insert("2".to_string());
        data.set("Test", Value::HashSet(set)).unwrap();
        run(
            &mut builder,
            vec![
                ProtocolType::String("Test".to_string()),
                ProtocolType::String("1".to_string()),
                ProtocolType::String("2".to_string()),
                ProtocolType::String("3".to_string()),
                ProtocolType::String("4".to_string())
            ],
            data.clone(),
        )
        .unwrap();

        assert!(data.contains_key("Test".to_string()));
        assert_eq!(":2\r\n", builder.serialize());
    }

    #[test]
    fn sadd_err_over_not_set_value() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        data.set("Test", Value::String("".to_string())).unwrap();
        let res = run(
            &mut builder,
            vec![
                ProtocolType::String("Test".to_string()),
                ProtocolType::String("1".to_string()),
                ProtocolType::String("2".to_string()),
                ProtocolType::String("3".to_string())
            ],
            data.clone(),
        );

        assert!(res.is_err());
    }

    #[test]
    fn sadd_over_already_setted_set() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        let mut set: HashSet<String> = HashSet::new();
        set.insert("1".to_string());
        data.set("Test", Value::HashSet(set)).unwrap();

        run(
            &mut builder,
            vec![
                ProtocolType::String("Test".to_string()),
                ProtocolType::String("1".to_string()),
            ],
            data.clone(),
        )
        .unwrap();

        assert_eq!(":0\r\n", builder.serialize());
    }
}
