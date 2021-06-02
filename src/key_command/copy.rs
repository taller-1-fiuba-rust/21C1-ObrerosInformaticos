use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use std::sync::Arc;
use crate::storage::data_storage::{DataStorage};

pub fn run(db: Arc<DataStorage>, arguments: Vec<ProtocolType>, builder: &mut ResponseBuilder) -> Result<(), &'static str>  {
    assert_eq!(arguments.len(), 2);

    let src = arguments[0].clone().string()?;
    let dst = arguments[1].clone().string()?;

    let read_lock = db.read();
    let value = read_lock.get(&src);
    let mut result = 0;
    if value.is_some() {
        let new_val = (&value.unwrap().1).clone();
        db.add_key_value(&dst, new_val);
        result = 1;
    }

    builder.add(ProtocolType::Integer(result));
    Ok(())
}