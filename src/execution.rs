use crate::protocol::command::Command;
use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;

pub struct Execution;

/*
    Execution should map each command name to a function which can execute it. They don't have to necessarily be located here
*/
impl Execution {
    pub fn run(cmd: &Command, builder: &mut ResponseBuilder) -> Result<(), &'static str> {
        match &cmd.name().to_ascii_lowercase()[..] {
            "ping" => Self::pong(builder),
            _ => Err("Unknown command."),
        }
    }

    pub fn pong(builder: &mut ResponseBuilder) -> Result<(), &'static str> {
        builder.add(ProtocolType::String("PONG".to_string()));
        Ok(())
    }
}
