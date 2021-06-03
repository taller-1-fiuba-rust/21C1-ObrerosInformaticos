use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use crate::storage::data_storage::DataStorage;
use std::sync::Arc;

pub fn run(
    db: Arc<DataStorage>,
    arguments: Vec<ProtocolType>,
    builder: &mut ResponseBuilder,
) -> Result<(), &'static str> {
    assert_eq!(arguments.len(), 2);

    let src = arguments[0].clone().string()?;
    let dst = arguments[1].clone().string()?;

    db.rename(&src, &dst)?;
    builder.add(ProtocolType::SimpleString("OK".to_string()));
    Ok(())
}
