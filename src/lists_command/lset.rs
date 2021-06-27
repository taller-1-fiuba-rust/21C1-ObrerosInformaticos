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

    let result = data.lset(key, index, value);

    match result {
        Ok(_) => {
            builder.add(ProtocolType::String("OK".to_string()));
            Ok(())
        }
        Err(s) => {
            builder.add(ProtocolType::Error(s.to_string()));
            Err(s)
        }
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
    fn lset_value_correct() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        data.set(
            "Test",
            Value::Vec(["1".to_string(), "2".to_string()].to_vec()),
        )
        .unwrap();

        run(
            &mut builder,
            vec![
                ProtocolType::String("Test".to_string()),
                ProtocolType::String("0".to_string()),
                ProtocolType::String("new_value".to_string()),
            ],
            data.clone(),
        )
        .unwrap();

        let value = data.get("Test").unwrap();

        let vector = match value {
            Value::Vec(i) => Ok(i),
            _ => Err("not vector value"),
        };

        assert_eq!("$2\r\nOK\r\n", builder.serialize());
        assert_eq!("new_value", vector.unwrap()[0]);
    }

    #[test]
    fn lset_negative_value_correct() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        data.set(
            "Test",
            Value::Vec(["1".to_string(), "2".to_string()].to_vec()),
        )
        .unwrap();

        run(
            &mut builder,
            vec![
                ProtocolType::String("Test".to_string()),
                ProtocolType::String("-1".to_string()),
                ProtocolType::String("new_value".to_string()),
            ],
            data.clone(),
        )
        .unwrap();

        let value = data.get("Test").unwrap();

        let vector = match value {
            Value::Vec(i) => Ok(i),
            _ => Err("not vector value"),
        };

        assert_eq!("$2\r\nOK\r\n", builder.serialize());
        assert_eq!("new_value", vector.unwrap()[1]);
    }

    #[test]
    #[should_panic]
    fn lset_value_not_correct() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        data.set(
            "Test",
            Value::Vec(["1".to_string(), "2".to_string()].to_vec()),
        )
        .unwrap();

        run(
            &mut builder,
            vec![
                ProtocolType::String("Test".to_string()),
                ProtocolType::String("5".to_string()),
                ProtocolType::String("new_value".to_string()),
            ],
            data.clone(),
        )
        .unwrap();
    }

    #[test]
    #[should_panic]
    fn lset_negative_value_not_correct() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        data.set(
            "Test",
            Value::Vec(["1".to_string(), "2".to_string()].to_vec()),
        )
        .unwrap();

        run(
            &mut builder,
            vec![
                ProtocolType::String("Test".to_string()),
                ProtocolType::String("-3".to_string()),
                ProtocolType::String("new_value".to_string()),
            ],
            data.clone(),
        )
        .unwrap();
    }
}
