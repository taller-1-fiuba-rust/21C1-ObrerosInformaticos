use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use crate::storage::data_storage::DataStorage;
use std::sync::Arc;
use std::usize;

use crate::storage::data_storage::Value;

pub fn run(
    arguments: Vec<ProtocolType>,
    builder: &mut ResponseBuilder,
    data: Arc<DataStorage>,
) -> Result<(), &'static str> {

    data.set(
        "test",
        Value::Vec(["value1".to_string(), "value2".to_string()].to_vec()),
    )
    .unwrap();

    if arguments.len() > 2 || arguments.is_empty() {
        return Err("ERR wrong number of arguments");
    }

    let key = arguments[0].clone().string()?;
    let mut count = 1;
    if arguments.len() > 1 {
        count = arguments[1].clone().integer()?;
    }

    let vals = data.lpop(key, count as usize)?;
    let res = if vals.is_empty() {
        ProtocolType::Nil()
    } else if vals.len() == 1 {
        ProtocolType::String(vals[0].clone())
    } else {
        ProtocolType::Array(
            vals.into_iter()
                .map(ProtocolType::String)
                .collect::<Vec<ProtocolType>>(),
        )
    };
    builder.add(res);
    Ok(())
}


#[cfg(test)]

mod tests {
    use super::*;
    use crate::storage::data_storage::Value;
    use std::collections::HashSet;

    #[test]
    fn test_lindex_of_vec_returns_ok() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        data.set(
            "test",
            Value::Vec(["value1".to_string(), "value2".to_string()].to_vec()),
        )
        .unwrap();

        run(
            vec![
                ProtocolType::String("test".to_string()),
                ProtocolType::String("1".to_string()),
            ],
            &mut builder,
            data.clone(),
        )
        .unwrap();

        assert_eq!("$6\r\nvalue2\r\n", builder.serialize());
    }

}
