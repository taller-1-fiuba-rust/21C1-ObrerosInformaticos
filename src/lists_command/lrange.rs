use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use crate::storage::data_storage::DataStorage;
use std::sync::Arc;

/// Returns the specified elements of the list stored at key. 
/// The offsets start and stop are zero-based indexes, with 0 being the first element of the list 
/// (the head of the list), 1 being the next element and so on.
/// These offsets can also be negative numbers indicating offsets starting at the end of the list.
pub fn run(
    builder: &mut ResponseBuilder,
    arguments: Vec<ProtocolType>,
    data: Arc<DataStorage>,
) -> Result<(), &'static str> {
    if arguments.len() != 3 {
        return Err("Wrong quantity of arguments.");
    }

    let key = arguments[0].clone().string()?;
    let first_index = arguments[1].clone().integer()?;
    let second_index = arguments[2].clone().integer()?;

    let values = data.lrange(key, first_index, second_index);

    match values {
        Ok(val) => match val {
            Some(vec_values) => {
                builder.add(ProtocolType::Array(
                    vec_values.into_iter().map(ProtocolType::String).collect(),
                ));
                Ok(())
            }
            None => {
                builder.add(ProtocolType::String("(empty list)".to_string()));
                Ok(())
            }
        },
        Err(s) => {
            builder.add(ProtocolType::Error(s.to_string()));
            Err(s)
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::protocol::parser::array::ArrayParser;
    use crate::protocol::parser::ProtocolParser;
    use crate::protocol::types::ProtocolType;
    use crate::storage::data_storage::DataStorage;
    use crate::storage::data_storage::Value;
    use std::sync::Arc;

    #[test]
    fn lrange_positive_index() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        data.set(
            "Test",
            Value::Vec(["1".to_string(), "2".to_string(), "3".to_string()].to_vec()),
        )
        .unwrap();

        run(
            &mut builder,
            vec![
                ProtocolType::String("Test".to_string()),
                ProtocolType::String("0".to_string()),
                ProtocolType::String("5".to_string()),
            ],
            data.clone(),
        )
        .unwrap();

        assert_response(&builder, vec!["1", "2", "3"]);
    }

    #[test]
    fn lrange_start_negative() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        data.set(
            "Test",
            Value::Vec(["1".to_string(), "2".to_string(), "3".to_string()].to_vec()),
        )
        .unwrap();

        run(
            &mut builder,
            vec![
                ProtocolType::String("Test".to_string()),
                ProtocolType::String("-1".to_string()),
                ProtocolType::String("5".to_string()),
            ],
            data.clone(),
        )
        .unwrap();

        assert_response(&builder, vec!["3"]);
    }

    #[test]
    fn lrange_stop_negative() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        data.set(
            "Test",
            Value::Vec(["1".to_string(), "2".to_string(), "3".to_string()].to_vec()),
        )
        .unwrap();

        run(
            &mut builder,
            vec![
                ProtocolType::String("Test".to_string()),
                ProtocolType::String("2".to_string()),
                ProtocolType::String("-1".to_string()),
            ],
            data.clone(),
        )
        .unwrap();

        assert_response(&builder, vec!["3"]);
    }

    #[test]
    fn lrange_negative_index() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        data.set(
            "Test",
            Value::Vec(["1".to_string(), "2".to_string(), "3".to_string()].to_vec()),
        )
        .unwrap();

        run(
            &mut builder,
            vec![
                ProtocolType::String("Test".to_string()),
                ProtocolType::String("-2".to_string()),
                ProtocolType::String("-1".to_string()),
            ],
            data.clone(),
        )
        .unwrap();

        assert_response(&builder, vec!["2", "3"]);
    }

    fn assert_response(builder: &ResponseBuilder, mut expected: Vec<&str>) {
        let mut parser = ArrayParser::new();

        for line in builder.serialize().split("\r\n") {
            println!("{}", line);
            if parser.feed(&format!("{}\r\n", line)).unwrap() {
                break;
            }
        }
        if let ProtocolType::Array(results) = parser.build() {
            let mut sorted_result: Vec<String> =
                results.into_iter().map(|x| x.string().unwrap()).collect();
            sorted_result.sort();
            expected.sort();
            assert_eq!(sorted_result, expected);
        } else {
            unreachable!();
        }
    }
}
