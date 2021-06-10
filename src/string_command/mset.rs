use crate::protocol::types::ProtocolType;
use crate::protocol::response::ResponseBuilder;
use crate::storage::data_storage::{DataStorage, Value};
use std::sync::Arc;

pub fn run(
    db: Arc<DataStorage>,
    arguments: Vec<ProtocolType>,
    builder: &mut ResponseBuilder,
) -> Result<(), &'static str> {
    let mut names = vec![];
    let mut values = vec![];
    let mut i = 0;
    for argument in arguments {
        let str = argument.clone().string()?;
        if i % 2 == 0 {
            names.push(str);
        } else {
            values.push(Value::String(str));
        }
        i += 1;
    }

    db.set_multiple(names, values)?;

    builder.add(ProtocolType::SimpleString("OK".to_string()));
    Ok(())
}
