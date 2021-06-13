use crate::protocol::types::ProtocolType;

///
/// Representation of a RESP command. e.g. The command "SET key value" will be equivalent to
///
pub struct Command {
    symbol: String,
    arguments: Vec<ProtocolType>,
}

impl Command {
    /// Create a new command
    pub fn new(symbol: String, arguments: Vec<ProtocolType>) -> Self {
        Command { symbol, arguments }
    }

    /// Returns the name of the command
    pub fn name(&self) -> String {
        self.symbol.clone()
    }

    /// Returns the list of arguments of the command, without the name.
    pub fn arguments(&self) -> Vec<ProtocolType> {
        self.arguments.clone()
    }
}
