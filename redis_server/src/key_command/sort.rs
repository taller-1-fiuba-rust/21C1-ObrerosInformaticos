use redis_protocol::response::ResponseBuilder;
use redis_protocol::types::ProtocolType;
use crate::storage::data_storage::DataStorage;
use crate::storage::data_storage::Value;
use std::sync::Arc;

/// Returns a sorted vec stored at the given key. If the keys holds a string value
/// it returns an error.
/// It has two optionals params:[desc -> descendent, store -> to store the result at the specified key]
pub fn run(
    builder: &mut ResponseBuilder,
    arguments: Vec<ProtocolType>,
    data: &Arc<DataStorage>,
) -> Result<(), &'static str> {
    let mut sort_values: Option<Vec<String>> = None;
    let key = arguments[0].clone().string()?;

    if arguments.len() == 1 {
        sort_values = Some(basic_sort(key, data)?);
    } else {
        for i in 1..arguments.len() {
            let second_argument: &str = &arguments[i].to_string().to_ascii_lowercase()[..];

            match second_argument {
                "desc" => {
                    sort_values = Some(inverse_sort(key.clone(), data)?);
                }
                "store" => {
                    if i == arguments.len() - 1 {
                        return Err("No new key specified.");
                    }
                    let values = basic_sort(key.clone(), data)?;
                    let new_key = arguments[i + 1].clone().string()?;
                    data.set(&new_key, Value::Vec(values.clone()))?;
                    sort_values = Some(values);
                }
                _ => (),
            }
        }
    }

    if let Some(vec) = sort_values {
        send_result(builder, vec)
    } else {
        return Err("Wrong arguments");
    }
    Ok(())
}

fn basic_sort(key: String, data: &Arc<DataStorage>) -> Result<Vec<String>, &'static str> {
    let mut values = get_values(data, key)?;
    values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let new_values = values.into_iter().map(|x| x.to_string()).collect();
    Ok(new_values)
}

fn inverse_sort(key: String, data: &Arc<DataStorage>) -> Result<Vec<String>, &'static str> {
    let mut values = get_values(data, key)?;
    values.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let new_values = values.into_iter().map(|x| x.to_string()).collect();
    Ok(new_values)
}

fn get_values(data: &Arc<DataStorage>, key: String) -> Result<Vec<f64>, &'static str> {
    let values = data.get(&key);
    match values {
        None => Err("None"),
        Some(Value::String(_)) => {
            Err("WRONGTYPE Operation against a key holding the wrong kind of value")
        }
        Some(Value::Vec(vec)) => return parse_to_int(vec),
        Some(Value::HashSet(set)) => {
            let mut sorted_vec = Vec::new();
            for element in set.iter() {
                sorted_vec.push(element.clone());
            }
            return parse_to_int(sorted_vec);
        }
    }
}

fn parse_to_int(vec: Vec<String>) -> Result<Vec<f64>, &'static str> {
    let mut new_vec: Vec<f64> = Vec::new();
    for i in vec.into_iter() {
        match i.parse() {
            Ok(nmb) => new_vec.push(nmb),
            Err(_) => return Err("ERR One or more scores can't be converted into double"),
        }
    }
    Ok(new_vec)
}

fn send_result(builder: &mut ResponseBuilder, values: Vec<String>) {
    let mut protocol_vec = Vec::new();
    for element in values.iter() {
        protocol_vec.push(ProtocolType::String(element.to_string()));
    }
    builder.add(ProtocolType::Array(protocol_vec));
}

#[cfg(test)]

mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_unnable_to_sort_string() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();

        data.set("key", Value::String("inexistent".to_string()))
            .unwrap();

        let run_result = run(
            &mut builder,
            vec![ProtocolType::String("key".to_string())],
            &data.clone(),
        );

        match run_result {
            Ok(_) => {
                assert_eq!(true, false)
            }
            Err(msj) => {
                assert_eq!(
                    msj,
                    "WRONGTYPE Operation against a key holding the wrong kind of value"
                )
            }
        }
    }

    #[test]
    fn test_sort_vec() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        let mut vc = Vec::new();

        vc.push("1".to_string());
        vc.push("10".to_string());
        vc.push("5".to_string());
        vc.push("30".to_string());
        vc.push("100".to_string());

        data.set("key", Value::Vec(vc)).unwrap();

        run(
            &mut builder,
            vec![ProtocolType::String("key".to_string())],
            &data.clone(),
        )
        .unwrap();

        assert_eq!(
            builder.serialize(),
            "*5\r\n$1\r\n1\r\n$1\r\n5\r\n$2\r\n10\r\n$2\r\n30\r\n$3\r\n100\r\n"
        );
    }

    #[test]
    fn test_sort_hashset() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        let mut vc = HashSet::new();

        vc.insert("asd".to_string());
        vc.insert("1".to_string());
        vc.insert("3".to_string());
        vc.insert("bsd".to_string());
        vc.insert("2".to_string());

        data.set("key", Value::HashSet(vc)).unwrap();

        let result = run(
            &mut builder,
            vec![ProtocolType::String("key".to_string())],
            &data.clone(),
        );

        match result {
            Ok(_) => assert!(false),
            Err(msg) => assert_eq!(msg, "ERR One or more scores can't be converted into double"),
        }
    }

    #[test]
    fn test_unnable_to_sort_unexistent_key() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();

        let run_result = run(
            &mut builder,
            vec![ProtocolType::String("key".to_string())],
            &data.clone(),
        );

        match run_result {
            Ok(_) => {
                assert_eq!(true, false)
            }
            Err(msj) => {
                assert_eq!(msj, "None")
            }
        }
    }
}
