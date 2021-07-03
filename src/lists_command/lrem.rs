use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use crate::storage::data_storage::DataStorage;
use std::sync::Arc;

pub fn run(
    builder: &mut ResponseBuilder,
    arguments: Vec<ProtocolType>,
    data: Arc<DataStorage>,
) -> Result<(), &'static str> {
    if arguments.len() != 3 {
        return Err("Wrong quantity of arguments.");
    }

    let key = arguments[0].clone().string()?;
    let index = arguments[1].clone().integer()?;
    let value = arguments[2].clone().string()?;

    let result = data.lrem(key, index, value);

    match result {
        Ok(s) => {
            builder.add(ProtocolType::SimpleString(s.to_string()));
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
    fn lrem_last_values_correct() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        data.set(
            "Test",
            Value::Vec(["1".to_string(), "2".to_string(), "2".to_string()].to_vec()),
        )
        .unwrap();

        run(
            &mut builder,
            vec![
                ProtocolType::String("Test".to_string()),
                ProtocolType::String("-2".to_string()),
                ProtocolType::String("2".to_string()),
            ],
            data.clone(),
        )
        .unwrap();

        let value = data.get("Test").unwrap();

        let vector = match value {
            Value::Vec(i) => Ok(i),
            _ => Err("not vector value"),
        };


        assert_eq!("+2\r\n", builder.serialize());
        assert_eq!(true, vec_compare(&vector.unwrap(), &["1".to_string()].to_vec()));
    }

    #[test]
    fn lrem_first_value_correct() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        data.set(
            "Test",
            Value::Vec(["1".to_string(), "1".to_string(), "2".to_string()].to_vec()),
        )
        .unwrap();

        run(
            &mut builder,
            vec![
                ProtocolType::String("Test".to_string()),
                ProtocolType::String("1".to_string()),
                ProtocolType::String("1".to_string()),
            ],
            data.clone(),
        )
        .unwrap();

        let value = data.get("Test").unwrap();

        let vector = match value {
            Value::Vec(i) => Ok(i),
            _ => Err("not vector value"),
        };


        assert_eq!("+1\r\n", builder.serialize());
        assert_eq!(true, vec_compare(&vector.unwrap(), &["1".to_string(), "2".to_string()].to_vec()));
    }

    #[test]
    fn lrem_all_values() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        data.set(
            "Test",
            Value::Vec(["1".to_string(), "1".to_string(), "2".to_string()].to_vec()),
        )
        .unwrap();

        run(
            &mut builder,
            vec![
                ProtocolType::String("Test".to_string()),
                ProtocolType::String("0".to_string()),
                ProtocolType::String("1".to_string()),
            ],
            data.clone(),
        )
        .unwrap();

        let value = data.get("Test").unwrap();

        let vector = match value {
            Value::Vec(i) => Ok(i),
            _ => Err("not vector value"),
        };


        assert_eq!("+2\r\n", builder.serialize());
        assert_eq!(true, vec_compare(&vector.unwrap(), &["2".to_string()].to_vec()));
    }

    fn vec_compare(va: &Vec<String>, vb: &Vec<String>) -> bool {
    (va.len() == vb.len()) &&
     va.iter()
       .zip(vb)
       .all(|(a,b)| (a == b))
    }
}