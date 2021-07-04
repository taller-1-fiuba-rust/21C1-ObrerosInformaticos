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
        Err("Wrong quantity of arguments")
    }
    let key = arguments[0].clone().string()?;

    let string_arguments: Vec<String> = arguments
        .into_iter()
        .map(|x| x.string())
        .collect::<Result<_, _>>()?;

    let result = data.srem(key, string_arguments[1..].to_owned());

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
    fn srem_value_correct() {
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
    fn srem_value_not_correct() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();

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
