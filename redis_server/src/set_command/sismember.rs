use crate::storage::data_storage::DataStorage;
use redis_protocol::response::ResponseBuilder;
use redis_protocol::types::ProtocolType;
use std::sync::Arc;

/// Returns if member is a member of the set stored at key.
/// 1 if the element is a member of the set.
/// 0 if the element is not a member of the set, or if key does not exist.
pub fn run(
    builder: &mut ResponseBuilder,
    arguments: Vec<ProtocolType>,
    data: Arc<DataStorage>,
) -> Result<(), &'static str> {
    if arguments.len() != 2 {
        return Err("Wrong quantity of arguments.");
    }

    let key = arguments[0].clone().string()?;
    let value = arguments[1].clone().string()?;

    let result = data.sismember(key, value);

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
    use crate::storage::data_storage::DataStorage;
    use crate::storage::data_storage::Value;
    use redis_protocol::types::ProtocolType;
    use std::collections::HashSet;
    use std::sync::Arc;

    #[test]
    fn lsismember_value_correct() {
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

        assert_eq!(":1\r\n", builder.serialize());
    }

    #[test]
    fn lsismember_value_not_correct() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        let mut set: HashSet<String> = HashSet::new();
        set.insert("1".to_string());
        data.set("Test", Value::HashSet(set)).unwrap();

        run(
            &mut builder,
            vec![
                ProtocolType::String("Test".to_string()),
                ProtocolType::String("2".to_string()),
            ],
            data.clone(),
        )
        .unwrap();

        assert_eq!(":0\r\n", builder.serialize());
    }
}
