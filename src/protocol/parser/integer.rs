use crate::protocol::parser::ProtocolParser;
use crate::protocol::types::ProtocolType;

///
/// Parses a serialized RESP signed integer into a ProtocolType::Integer
///
pub struct IntegerParser {
    data: i64,
}

impl IntegerParser {
    pub fn new() -> Self {
        IntegerParser { data: 0 }
    }
}

impl ProtocolParser for IntegerParser {
    fn get_prefix(&self) -> char {
        ':'
    }

    fn feed(&mut self, line: &str) -> Result<bool, String> {
        let len = line.len();
        let slice_result = line[1..len - 2].to_string();
        match slice_result.parse() {
            Ok(val) => {
                self.data = val;
                Ok(true)
            }
            Err(_) => Err(format!("Invalid '{}' integer received.", slice_result)),
        }
    }

    fn build(&self) -> ProtocolType {
        ProtocolType::Integer(self.data)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_integer() {
        let sample = ":54\r\n".to_string();
        let mut parser = IntegerParser::new();

        assert!(parser.feed(&sample).unwrap());

        let result = parser.build().integer().unwrap();
        assert_eq!(result, 54);
    }

    #[test]
    fn test_parse_negative_integer() {
        let sample = ":-32\r\n".to_string();
        let mut parser = IntegerParser::new();

        assert!(parser.feed(&sample).unwrap());

        let result = parser.build().integer().unwrap();
        assert_eq!(result, -32);
    }
}
