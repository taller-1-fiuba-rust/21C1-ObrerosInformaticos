use crate::protocol::command::Command;
use crate::protocol::response::ResponseBuilder;
use crate::storage::data_storage::DataStorage;
use crate::protocol::types::ProtocolType;
use std::time::SystemTime;
use std::time::Duration;
use std::sync::Arc;

pub fn set_expiration_to_key(builder: &mut ResponseBuilder, cmd: &Command, data: &Arc<DataStorage>) -> Result<(), &'static str> {

	let arguments: Vec<ProtocolType> = cmd.arguments();
	assert_eq!(arguments.len(), 2);

	let key = match arguments[0].clone().string() {
        Ok(s) => s,
        Err(_s) => {
            return Err("While parsing key in set_expiration in expire command");
        }
    };

    let seconds = match arguments[1].clone().string() {
        Ok(s) => s,
        Err(_s) => {
			println!("{:?}", _s);
            return Err("While parsing seconds in set_expiration in expire command");
        }
    };

    let secs: u64 = seconds.parse().unwrap();

	match data.set_expiration_to_key(SystemTime::now(), Duration::from_secs(secs), &key){
		Ok(s) => builder.add(ProtocolType::Integer(s as i32)),
		Err(_s) => builder.add(ProtocolType::Integer(0)),
	};

	Ok(())
}