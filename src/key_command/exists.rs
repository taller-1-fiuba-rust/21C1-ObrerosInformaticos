use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use crate::storage::data_storage::DataStorage;
use std::sync::Arc;

pub fn run(
    builder: &mut ResponseBuilder,
    arguments: Vec<ProtocolType>,
    data: &Arc<DataStorage>,
) -> Result<(), &'static str> {
	let mut string_arguments = vec![];

    for argument in arguments {
        match argument.clone().string() {
            Ok(s) => string_arguments.push(s),
            Err(_s) => {
                return Err("While parsing argument in exists command");
            }
        };
    }

    let mut counter = 0;

    for arg in string_arguments {
        let result = data.exists_key(&arg);

        if result == Ok(()) {
            counter += 1;
        }
    }

    builder.add(ProtocolType::Integer(counter));
    Ok(())
}