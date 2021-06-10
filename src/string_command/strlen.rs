use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use crate::storage::data_storage::{DataStorage, Value};
use std::sync::Arc;

pub fn run(
    db: Arc<DataStorage>,
    arguments: Vec<ProtocolType>,
    builder: &mut ResponseBuilder,
) -> Result<(), &'static str> {
    assert_eq!(arguments.len(), 1);

    let key = arguments[0].clone().string()?;
    let maybe_val = db.get(&key);
    if let Some(val) = maybe_val {
        match val {
            Value::String(s) => builder.add(ProtocolType::Integer(s.len() as i64)),
            Value::HashSet(_) => return Err("Stored value is a hashset"),
            Value::Vec(_) => return Err("Stored value is a list"),
        }
    } else {
        builder.add(ProtocolType::Integer(0));
    }
    Ok(())
}
