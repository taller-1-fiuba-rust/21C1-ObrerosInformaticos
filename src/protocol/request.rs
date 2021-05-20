use crate::protocol::command::Command;
use crate::protocol::parser::array::ArrayParser;
use crate::protocol::parser::ProtocolParser;

pub struct Request {
    parser: ArrayParser,
}

impl Request {
    pub fn new() -> Self {
        Request {
            parser: ArrayParser::new(),
        }
    }

    pub fn feed(&mut self, line: &str) {
        self.parser.feed(line);
    }

    pub fn build(&self) -> Command {
        let mut types = self.parser.build().array();
        let symbol = types[0].clone().string();
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
            request.feed(&line.to_string());
        }

        let command = request.build();
        assert_eq!(command.name(), "LLEN".to_string());
        let args = command.arguments();
        assert_eq!(args.len(), 1);
        assert_eq!(args[0].clone().string(), "mylist");
    }
}
