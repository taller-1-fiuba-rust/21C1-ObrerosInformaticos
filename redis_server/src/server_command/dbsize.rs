use redis_protocol::response::ResponseBuilder;
use redis_protocol::types::ProtocolType;
use crate::storage::data_storage::DataStorage;
use std::sync::Arc;

///Returns the number of elements in the data base.
pub fn run(builder: &mut ResponseBuilder, data: Arc<DataStorage>) -> Result<(), &'static str> {
    let result = data.len();

    match result {
        Ok(s) => {
            builder.add(ProtocolType::Integer(s as i64));
            Ok(())
        }
        Err(_) => Err("can't count elements in data base"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::data_storage::Value;

    #[test]
    fn test_count() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        data.set("key1", Value::String("value1".to_string()))
            .unwrap();
        data.set("key2", Value::String("value2".to_string()))
            .unwrap();
        data.set("key3", Value::String("value3".to_string()))
            .unwrap();
        data.set("key4", Value::String("value4".to_string()))
            .unwrap();

        run(&mut builder, data.clone()).unwrap();

        assert_eq!(builder.serialize(), ":4\r\n");
    }
}
