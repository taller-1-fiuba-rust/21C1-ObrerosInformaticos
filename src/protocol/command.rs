use crate::types::ProtocolType;

pub struct Command {
    symbol: String,
    arguments: Vec<ProtocolType>
}

impl Command {
    pub fn new(symbol: String, arguments: Vec<ProtocolType>) -> Self {
        Command {
            symbol,
            arguments
        }
    }
}