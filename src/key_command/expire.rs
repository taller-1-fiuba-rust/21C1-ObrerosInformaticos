use crate::protocol::command::Command;
use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use crate::storage::data_storage::DataStorage;
use std::sync::Arc;
use std::time::Duration;

pub fn set_expiration_to_key(
    builder: &mut ResponseBuilder,
    cmd: &Command,
    data: &Arc<DataStorage>,
) -> Result<(), &'static str> {
    let arguments: Vec<ProtocolType> = cmd.arguments();
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

    if seconds.is_negative() {
        match data.delete_key(&key) {
            Ok(_s) => builder.add(ProtocolType::Integer(1)),
            Err(_s) => builder.add(ProtocolType::Integer(0)),
        }
    } else {
        match data.set_expiration_to_key(
            Duration::from_secs(seconds as u64),
            &key,
        ) {
            Ok(s) => builder.add(ProtocolType::Integer(s as i32)),
            Err(_s) => builder.add(ProtocolType::Integer(0)),
        };
    }

    Ok(())
}
