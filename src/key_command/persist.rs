use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use crate::storage::data_storage::DataStorage;
use std::sync::Arc;

pub fn run(
    db: Arc<DataStorage>,
    arguments: Vec<ProtocolType>,
    builder: &mut ResponseBuilder,
) -> Result<(), &'static str> {
    assert_eq!(arguments.len(), 1);

    let src = arguments[0].clone().string()?;

    let mut result = 0;
    let (duration_maybe, _) = db.get_with_expiration(&src).ok_or("Key not found")?;
    if duration_maybe.is_some() {
        result = 1;
        db.set_expiration_to_key(None, &src)?;
    }
    builder.add(ProtocolType::Integer(result));
    Ok(())
}

