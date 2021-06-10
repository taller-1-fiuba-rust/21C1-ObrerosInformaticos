use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use crate::storage::data_storage::DataStorage;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

///Sets the expiration time of a key by taking the absolute time of UNIX.
///If the time given is less than the current one, the key is removed from the data set.
pub fn run(
    builder: &mut ResponseBuilder,
    arguments: Vec<ProtocolType>,
    data: &Arc<DataStorage>,
) -> Result<(), &'static str> {
    assert_eq!(arguments.len(), 2);

    let key = match arguments[0].clone().string() {
        Ok(s) => s,
        Err(_s) => {
            return Err("While parsing key in set_expiration in expire command");
        }
    };

    let seconds = match arguments[1].clone().integer() {
        Ok(s) => s,
        Err(_s) => {
            println!("{:?}", _s);
            return Err("While parsing seconds in set_expiration in expire command");
        }
    };

    let actual_time = SystemTime::now().duration_since(UNIX_EPOCH).ok();
    let input_time = Duration::from_secs(seconds as u64);

    if actual_time > Some(input_time) {
        match data.delete_key(&key) {
            Ok(_s) => builder.add(ProtocolType::Integer(1)),
            Err(_s) => builder.add(ProtocolType::Integer(0)),
        }
    } else {
        let actual_time = SystemTime::now();
        let expiration_time = actual_time
            .checked_add(Duration::from_secs(seconds as u64))
            .ok_or("Failed to calculate expiration time")?
            .duration_since(UNIX_EPOCH)
            .ok()
            .ok_or("Failed to calculate expiration time")?;
        match data.set_expiration_to_key(Some(expiration_time), &key) {
            Ok(s) => builder.add(ProtocolType::Integer((s as i32).into())),
            Err(_s) => builder.add(ProtocolType::Integer(0)),
        };
    }

    Ok(())
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::protocol::types::ProtocolType;
    use crate::storage::data_storage::DataStorage;
    use crate::storage::data_storage::Value;
    use std::sync::Arc;

    #[test]
    fn set_expire_at_to_key() {
        let data = Arc::new(DataStorage::new());
        let mut builder = ResponseBuilder::new();
        data.set("mykey", Value::String("Hello".to_string()))
            .unwrap();

        run(
            &mut builder,
            vec![
                ProtocolType::String("mykey".to_string()),
                ProtocolType::String("1293840000".to_string()),
            ],
            &data.clone(),
        )
        .unwrap();

        assert_eq!("*1\r\n:1\r\n", builder.serialize());
    }
}
