use crate::protocol::command::Command;
use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use crate::storage::data_storage::DataStorage;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

///Dada una clave y un valor setea el valor como expiracion
///de la clave. Si el valor es negativo se elimina la clave de
///la estructura de datos.
pub fn run(
    builder: &mut ResponseBuilder,
    cmd: &Command,
    data: &Arc<DataStorage>,
) -> Result<(), &'static str> {
    let arguments: Vec<ProtocolType> = cmd.arguments();
    if arguments.len() != 2 {
        return Err("Wrong number of arguments");
    }

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
        let actual_time = SystemTime::now();
        let expiration_time = actual_time
            .checked_add(Duration::from_secs(seconds as u64))
            .ok_or("Failed to calculate expiration time")?
            .duration_since(UNIX_EPOCH)
            .ok()
            .ok_or("Failed to calculate expiration time")?;
        match data.set_expiration_to_key(Some(expiration_time), &key) {
            Ok(s) => builder.add(ProtocolType::Integer(s as i64)),
            Err(_s) => builder.add(ProtocolType::Integer(0)),
        };
    }

    Ok(())
}
