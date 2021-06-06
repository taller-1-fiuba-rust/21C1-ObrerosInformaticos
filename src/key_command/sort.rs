use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use crate::storage::data_storage::DataStorage;
use crate::storage::data_storage::Value;
use std::sync::Arc;

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
                    data.add_key_value(&new_key, Value::Vec(values.clone()))?;
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
    values.sort();
    Ok(values)
}

fn inverse_sort(key: String, data: &Arc<DataStorage>) -> Result<Vec<String>, &'static str> {
    let mut values = get_values(data, key)?;
    values.sort_by(|a, b| b.cmp(a));
    Ok(values)
}

fn get_values(data: &Arc<DataStorage>, key: String) -> Result<Vec<String>, &'static str> {
    let values = data.get(&key);
    match values {
        None => Err("None"),
        Some(Value::String(_)) => Err("String value. No possible sort."),
        Some(Value::Vec(vec)) => Ok(vec),
        Some(Value::HashSet(set)) => {
            let mut sorted_vec = Vec::new();
            for element in set.iter() {
                sorted_vec.push(element.clone());
            }
            Ok(sorted_vec)
        }
    }
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

        data.add_key_value("key", Value::String("inexistent".to_string()))
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
                assert_eq!(msj, "String value. No possible sort.")
            }
        }
    }

    #[test]
    fn test_sort_vec() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        let mut vc = Vec::new();

        vc.push("asd".to_string());
        vc.push("1".to_string());
        vc.push("3".to_string());
        vc.push("bsd".to_string());
        vc.push("2".to_string());

        data.add_key_value("key", Value::Vec(vc)).unwrap();

        run(
            &mut builder,
            vec![ProtocolType::String("key".to_string())],
            &data.clone(),
        )
        .unwrap();

        assert_eq!(
            builder.serialize(),
            "*1\r\n*5\r\n$1\r\n1\r\n$1\r\n2\r\n$1\r\n3\r\n$3\r\nasd\r\n$3\r\nbsd\r\n"
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

        data.add_key_value("key", Value::HashSet(vc)).unwrap();

        run(
            &mut builder,
            vec![ProtocolType::String("key".to_string())],
            &data.clone(),
        )
        .unwrap();

        assert_eq!(
            builder.serialize(),
            "*1\r\n*5\r\n$1\r\n1\r\n$1\r\n2\r\n$1\r\n3\r\n$3\r\nasd\r\n$3\r\nbsd\r\n"
        );
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
