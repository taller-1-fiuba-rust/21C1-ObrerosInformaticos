use crate::command::Command;
use crate::parser::array::ArrayParser;
use crate::parser::ProtocolParser;

/// Parses a RESP command request line by line.
pub struct Request {
    parser: ArrayParser,
}

impl Request {
    /// Create a new request parser
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Request {
            parser: ArrayParser::new(),
        }
    }

    /// Feed a line to the internal parser
    pub fn feed(&mut self, line: &str) -> Result<bool, String> {
        self.parser.feed(line)
    }

    /// Build a new command from the parsed request.
    pub fn build(&self) -> Command {
        let mut types = self.parser.build().array().unwrap();
        let symbol = types[0].clone().string().unwrap();
        types.remove(0);
        Command::new(symbol, types)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_request() {
        let sample_request = vec!["*2\r\n", "$4\r\n", "LLEN\r\n", "$6\r\n", "mylist\r\n"];
        let mut request = Request::new();

        for line in sample_request {
            request.feed(&line.to_string()).unwrap();
        }

        let command = request.build();
        assert_eq!(command.name(), "LLEN".to_string());
        let args = command.arguments();
        assert_eq!(args.len(), 1);
        assert_eq!(args[0].clone().string().unwrap(), "mylist");
    }
}
