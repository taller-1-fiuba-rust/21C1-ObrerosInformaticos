use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use crate::storage::data_storage::DataStorage;
use std::sync::Arc;

/// Returns all the members of the set value stored at key.
pub fn run(
    builder: &mut ResponseBuilder,
    arguments: Vec<ProtocolType>,
    data: Arc<DataStorage>,
) -> Result<(), &'static str> {
    if arguments.len() != 1 {
        return Err("Wrong quantity of arguments.");
    }

    let key = arguments[0].clone().string()?;
    let result = data.smember(key);

    match result {
        Ok(s) => {
            builder.add(ProtocolType::Array(
                s.into_iter().map(ProtocolType::String).collect(),
            ));
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
    fn smember_values() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        let mut set: HashSet<String> = HashSet::new();
        set.insert("correct".to_string());
        data.set("Test", Value::HashSet(set)).unwrap();

        run(
            &mut builder,
            vec![ProtocolType::String("Test".to_string())],
            data.clone(),
        )
        .unwrap();

        assert_eq!("*1\r\n$7\r\ncorrect\r\n", builder.serialize());
    }

    #[test]
    fn smember_not_value() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();

        run(
            &mut builder,
            vec![ProtocolType::String("Test".to_string())],
            data.clone(),
        )
        .unwrap();

        assert_eq!("*0\r\n", builder.serialize());
    }
}
