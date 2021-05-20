use crate::protocol::parser::ProtocolParser;
use crate::protocol::types::ProtocolType;

pub struct IntegerParser {
    data: i32,
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

    fn feed(&mut self, line: &str) -> bool {
        let len = line.len();
        self.data = line[1..len - 2].parse().unwrap();
        true
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

        assert!(parser.feed(&sample));

        let result = parser.build().integer();
        assert_eq!(result, 54);
    }
}
