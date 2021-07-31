use redis_protocol::response::ResponseBuilder;
use redis_protocol::types::ProtocolType;
use crate::storage::data_storage::DataStorage;
use std::sync::Arc;

///Delete all the keys of the currently selected DB.
pub fn run(builder: &mut ResponseBuilder, data: Arc<DataStorage>) -> Result<(), &'static str> {
    let response = data.delete_all();

    match response {
        Ok(_) => {
            builder.add(ProtocolType::String("OK".to_string()));
            Ok(())
        }
        Err(_) => Err("Flushdb not executed"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::data_storage::Value;

    #[test]
    fn test_delete_all_keys() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        data.set("key1", Value::String("value1".to_string()))
            .unwrap();
        data.set("key2", Value::String("value2".to_string()))
            .unwrap();

        run(&mut builder, data.clone()).unwrap();

        assert!(data.is_empty());
    }
}
