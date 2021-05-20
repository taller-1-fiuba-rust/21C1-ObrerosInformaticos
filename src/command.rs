struct Command {
    symbol: String,
    arguments: Vec<MixedType>
}

impl Command {
    pub fn new() -> Self {
        Command {}
    }
}