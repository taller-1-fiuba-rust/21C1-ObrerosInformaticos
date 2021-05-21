use crate::protocol::types::ProtocolType;

#[allow(dead_code)]
pub struct Command {
    symbol: String,
    arguments: Vec<ProtocolType>,
}

impl Command {
    pub fn new(symbol: String, arguments: Vec<ProtocolType>) -> Self {
        Command { symbol, arguments }
    }

    pub fn name(&self) -> String {
        self.symbol.clone()
    }

    pub fn arguments(&self) -> Vec<ProtocolType> {
        self.arguments.clone()
    }
}
