use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;

pub fn run(builder: &mut ResponseBuilder) -> Result<(), &'static str> {
    builder.add(ProtocolType::String("PONG".to_string()));
    Ok(())
}