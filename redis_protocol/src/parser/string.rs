use crate::parser::ProtocolParser;
use crate::types::ProtocolType;

///
/// Parses a serialized RESP simple string into a ProtocolType::SimpleString
///
pub struct SimpleStringParser {
    data: String,
}

impl SimpleStringParser {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleStringParser {
            data: String::new(),
        }
    }
}

impl ProtocolParser for SimpleStringParser {
    fn get_prefix(&self) -> char {
        '+'
    }

    fn feed(&mut self, line: &str) -> Result<bool, String> {
        let l = line.len();
        self.data = line[1..l - 2].to_string();
        Ok(true)
    }

    fn build(&self) -> ProtocolType {
        ProtocolType::SimpleString(self.data.clone())
    }
}

///
/// Parses a serialized RESP string into a ProtocolType::String
///
pub struct BulkStringParser {
    data: String,
    length: i32,
}

impl BulkStringParser {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        BulkStringParser {
            data: String::new(),
            length: 0,
        }
    }
}

impl ProtocolParser for BulkStringParser {
    fn get_prefix(&self) -> char {
        '$'
    }

    fn feed(&mut self, line: &str) -> Result<bool, String> {
        let len = line.len();
        assert!(len > 0);
        let symbol = line.chars().next().unwrap();
        if symbol == self.get_prefix() {
            let slice_result = line[1..len - 2].to_string();
            match slice_result.parse::<i32>() {
                Ok(val) => {
                    self.length = val;
                    Ok(matches!(val, -1))
                }
                Err(_) => Err(format!("Invalid '{}' length received.", slice_result)),
            }
        } else {
            self.data = line[0..self.length as usize].to_string();
            Ok(true)
        }
    }

    fn build(&self) -> ProtocolType {
        if self.length == -1 {
            return ProtocolType::Nil();
        }
        assert_eq!(self.length as usize, self.data.len());
        ProtocolType::String(self.data.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_string() {
        let sample = "+OK\r\n".to_string();
        let mut parser = SimpleStringParser::new();

        assert!(parser.feed(&sample).unwrap());

        let result = parser.build().clone().string().unwrap();
        assert_eq!(result, "OK");
    }

    #[test]
    fn test_parse_bulk_string() {
        let mut parser = BulkStringParser::new();

        assert!(!parser.feed(&"$22\r\n".to_string()).unwrap());
        assert!(parser
            .feed(&"Hi! I am a Bulk String\r\n".to_string())
            .unwrap());

        let result = parser.build().clone().string().unwrap();
        assert_eq!(result, "Hi! I am a Bulk String");
    }
}
