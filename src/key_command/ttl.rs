use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use crate::storage::data_storage::DataStorage;
use std::sync::Arc;

pub fn run(
    builder: &mut ResponseBuilder,
    arguments: Vec<ProtocolType>,
    data: &Arc<DataStorage>,
) -> Result<(), &'static str> {
    if arguments.len() != 1 {
        return Err("Wrong quantity of arguments. Command TTL has only one.");
    }

    let key = arguments[0].clone().string()?;

    let value_option = data.get_with_expiration(&key);

    match value_option {
        Some((duration_op, _)) => match duration_op {
            Some(duration) => builder.add(ProtocolType::Integer(duration.as_secs() as i64)),
            None => builder.add(ProtocolType::Integer(-1)),
        },
        None => builder.add(ProtocolType::Integer(-2)),
    }
    Ok(())
}
