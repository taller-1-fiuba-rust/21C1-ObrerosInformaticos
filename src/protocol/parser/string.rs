use crate::protocol::parser::ProtocolParser;
use crate::protocol::types::ProtocolType;

pub struct SimpleStringParser {
    data: String
}

impl SimpleStringParser {
    pub fn new() -> Self {
        SimpleStringParser { data: String::new() }
    }
}

impl ProtocolParser for SimpleStringParser {
    fn get_prefix(&self) -> char {
        return '+';
    }

    fn feed(&mut self, line: &String) -> bool {
        let l = line.len();
        self.data = line[1..l-2].to_string();
        return true;
    }

    fn build(&self) -> ProtocolType {
        return ProtocolType::String(self.data.clone());
    }
}

pub struct BulkStringParser {
    data: String,
    length: usize
}

impl BulkStringParser {
    pub fn new() -> Self {
        BulkStringParser {
            data: String::new(),
            length: 0
        }
    }
}

impl ProtocolParser for BulkStringParser {
    fn get_prefix(&self) -> char {
        return '$';
    }

    fn feed(&mut self, line: &String) -> bool {
        let len = line.len();
        assert!(len > 0);
        let symbol = line.chars().nth(0).unwrap();
        return if symbol == self.get_prefix() {
            self.length = line[1..len-2].parse().unwrap();
            false
        } else {
            self.data = line[0..self.length].to_string();
            true
        }
    }

    fn build(&self) -> ProtocolType {
        assert_eq!(self.length, self.data.len());
        return ProtocolType::String(self.data.clone());
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_string() {
        let sample = "+OK\r\n".to_string();
        let mut parser = SimpleStringParser::new();

        assert!(parser.feed(&sample));

        let result = parser.build().clone().string();
        assert_eq!(result, "OK");
    }

    #[test]
    fn test_parse_bulk_string() {
        let mut parser = BulkStringParser::new();

        assert!(!parser.feed(&"$22\r\n".to_string()));
        assert!(parser.feed(&"Hi! I am a Bulk String\r\n".to_string()));

        let result = parser.build().clone().string();
        assert_eq!(result, "Hi! I am a Bulk String");
    }
}
