use crate::protocol::parser::ProtocolParser;
use crate::protocol::parser::SimpleStringParser;
use crate::protocol::types::ProtocolType;

pub struct ErrorParser {
    parser: SimpleStringParser,
}

#[allow(dead_code)]
impl ErrorParser {
    pub fn new() -> Self {
        ErrorParser {
            parser: SimpleStringParser::new(),
        }
    }
}

impl ProtocolParser for ErrorParser {
    fn get_prefix(&self) -> char {
        '-'
    }

    fn feed(&mut self, line: &str) -> Result<bool, String> {
        self.parser.feed(line)
    }

    fn build(&self) -> ProtocolType {
        ProtocolType::Error(self.parser.build().string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_error() {
        let sample = "-ERR Exploto todo!\r\n".to_string();
        let mut parser = ErrorParser::new();

        assert!(parser.feed(&sample).unwrap());

        let result = parser.build().clone().error();
        assert_eq!(result, "ERR Exploto todo!");
    }
}
