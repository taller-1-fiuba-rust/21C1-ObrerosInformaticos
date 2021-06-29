use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use crate::storage::data_storage::DataStorage;
use std::sync::Arc;

pub fn run(
    builder: &mut ResponseBuilder,
    arguments: Vec<ProtocolType>,
    data: Arc<DataStorage>,
) -> Result<(), &'static str> {
    if arguments.len() > 2 || arguments.is_empty() {
        return Err("ERR wrong number of arguments");
    }

    let key = arguments[0].clone().string()?;
    let mut count = 1;
    if arguments.len() > 1 {
        count = arguments[1].clone().integer()?;
    }

    let vals = data.rpop(key, count as usize)?;
    let res = if vals.is_empty() {
        ProtocolType::Nil()
    } else if vals.len() == 1 {
        ProtocolType::String(vals[0].clone())
    } else {
        ProtocolType::Array(vals.into_iter().map(ProtocolType::String).collect::<Vec<ProtocolType>>())
    };
    builder.add(res);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::storage::data_storage::{DataStorage, Value};
    use crate::protocol::response::ResponseBuilder;
    use std::sync::Arc;
    use super::*;
    use crate::protocol::types::ProtocolType;

    #[test]
    fn pop_one() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        data.set("Test", Value::Vec(["1".to_string()].to_vec()))
            .unwrap();

        run(
            &mut builder,
            vec![
                ProtocolType::String("Test".to_string())
            ],
            data.clone(),
        )
            .unwrap();

        assert!(
            data.get("Test").unwrap().array().unwrap().is_empty()
        );
        assert_eq!("$1\r\n1\r\n", builder.serialize());
    }

    #[test]
    fn pop_multiple() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        data.set("Test", Value::Vec(["1".to_string(), "2".to_string(), "3".to_string(), "4".to_string(), "5".to_string()].to_vec()))
            .unwrap();

        run(
            &mut builder,
            vec![
                ProtocolType::String("Test".to_string()),
                ProtocolType::String("3".to_string()),
            ],
            data.clone(),
        )
            .unwrap();

        assert_eq!(
            vec!["1", "2"],
            data.get("Test").unwrap().array().unwrap()
        );
        assert_eq!(ProtocolType::Array(vec![
            ProtocolType::String("5".to_string()),
            ProtocolType::String("4".to_string()),
            ProtocolType::String("3".to_string()),
        ]).serialize(), builder.serialize());
    }

    #[test]
    fn pop_no_list() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();

        run(
            &mut builder,
            vec![
                ProtocolType::String("Test".to_string()),
            ],
            data.clone(),
        )
            .unwrap();

        assert_eq!(ProtocolType::Nil().serialize(), builder.serialize());
    }

    #[test]
    fn pop_empty_list() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        data.set("Test", Value::Vec([].to_vec()))
            .unwrap();
        run(
            &mut builder,
            vec![
                ProtocolType::String("Test".to_string()),
            ],
            data.clone(),
        )
            .unwrap();

        assert_eq!(ProtocolType::Nil().serialize(), builder.serialize());
    }
}
