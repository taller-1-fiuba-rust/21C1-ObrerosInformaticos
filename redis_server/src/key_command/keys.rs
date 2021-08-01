use crate::storage::data_storage::DataStorage;
use redis_protocol::response::ResponseBuilder;
use redis_protocol::types::ProtocolType;
use regex::Regex;
use std::sync::Arc;

/// Returns all keys matching pattern.
pub fn run(
    db: Arc<DataStorage>,
    arguments: Vec<ProtocolType>,
    builder: &mut ResponseBuilder,
) -> Result<(), &'static str> {
    if arguments.len() != 1 {
        return Err("Wrong number of arguments");
    }

    // We need to append ^$ to the regex in order to force the engine to match the start and end of the word.
    // Otherwise a pattern such as a?e will match any word with contains an 'a' followed by any char and then by an 'e'.
    // An example of this is lastname or firstname and we want it to only match age, aim or ate.
    let mut pattern_str = arguments[0].clone().string()?;
    // For matching a single character regex uses . instead of ?
    pattern_str = pattern_str.replace("?", ".");
    pattern_str = pattern_str.replace("*", ".*");
    let pattern = format!("^{}$", pattern_str);
    let re = Regex::new(&pattern).ok().ok_or("Error parsing the regex")?;
    let all_keys = db.get_keys();

    builder.add(ProtocolType::Array(
        all_keys
            .into_iter()
            .filter(move |x| re.is_match(x))
            .map(ProtocolType::String)
            .collect(),
    ));
    Ok(())
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::storage::data_storage::Value;
    use redis_protocol::parser::array::ArrayParser;
    use redis_protocol::parser::ProtocolParser;

    fn get_test_data() -> (Arc<DataStorage>, ResponseBuilder) {
        let data = Arc::new(DataStorage::new());
        for x in vec![
            "firstname",
            "Jack",
            "lastname",
            "Stuntman",
            "age",
            "aim",
            "ate",
            "abe",
        ] {
            data.set(x, Value::String("value".to_string())).unwrap();
        }
        return (data, ResponseBuilder::new());
    }

    fn run_command(data: Arc<DataStorage>, builder: &mut ResponseBuilder, pattern: &str) {
        run(
            data.clone(),
            vec![ProtocolType::String(pattern.to_string())],
            builder,
        )
        .unwrap();
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

    #[test]
    fn test_keys_any_length() {
        let (data, mut builder) = get_test_data();

        run_command(data, &mut builder, "*name*");
        assert_response(&builder, vec!["firstname", "lastname"]);
    }

    #[test]
    fn test_keys_any_char() {
        let (data, mut builder) = get_test_data();

        run_command(data, &mut builder, "a?e");
        assert_response(&builder, vec!["age", "ate", "abe"]);
    }

    #[test]
    fn test_keys_incomplete() {
        let (data, mut builder) = get_test_data();

        run_command(data, &mut builder, "Stunt");
        assert_response(&builder, vec![]);
    }

    #[test]
    fn test_keys_same() {
        let (data, mut builder) = get_test_data();

        run_command(data, &mut builder, "Jack");
        assert_response(&builder, vec!["Jack"]);
    }

    #[test]
    fn test_keys_set() {
        let (data, mut builder) = get_test_data();

        run_command(data, &mut builder, "a[gt]e");
        assert_response(&builder, vec!["ate", "age"]);
    }

    #[test]
    fn test_keys_range() {
        let (data, mut builder) = get_test_data();

        run_command(data, &mut builder, "a[a-z]e");
        assert_response(&builder, vec!["ate", "age", "abe"]);
    }

    #[test]
    fn test_keys_except() {
        let (data, mut builder) = get_test_data();

        run_command(data, &mut builder, "a[^g]e");
        assert_response(&builder, vec!["ate", "abe"]);
    }
}
