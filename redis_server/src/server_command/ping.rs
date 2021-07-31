use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;

///Funcion para la ejecucion del comando PING.
///Imprime por pantalla PONG ante la ejecucion del comando.
pub fn run(builder: &mut ResponseBuilder) -> Result<(), &'static str> {
    builder.add(ProtocolType::String("PONG".to_string()));
    Ok(())
}
