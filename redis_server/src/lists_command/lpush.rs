use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use crate::storage::data_storage::DataStorage;
use std::sync::Arc;

/// Insert all the specified values at the head of the list stored at key.
/// If key does not exist, it is created as empty list before performing the push operations.
/// When key holds a value that is not a list, an error is returned.
/// It is possible to push multiple elements using a single command call just specifying multiple arguments at the end of the command.
/// Elements are inserted one after the other to the head of the list, from the leftmost element to the rightmost element.
/// So for instance the command LPUSH mylist a b c will result into a list containing c as first element, b as second element and a as third element.
pub fn run(
    builder: &mut ResponseBuilder,
    arguments: Vec<ProtocolType>,
    data: Arc<DataStorage>,
) -> Result<(), &'static str> {
    if arguments.len() < 2 {
        return Err("ERR wrong number of arguments for 'lpush' command");
    }

    let mut string_arguments: Vec<String> = arguments
        .into_iter()
        .map(|x| x.string())
        .collect::<Result<_, _>>()?;

    let key = string_arguments[0].clone();
    string_arguments.remove(0);

    let list_len = data.lpush(key, string_arguments);

    match list_len {
        Ok(len) => {
            builder.add(ProtocolType::Integer(len as i64));
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

        assert_eq!(":1\r\n", builder.serialize());
    }
}
