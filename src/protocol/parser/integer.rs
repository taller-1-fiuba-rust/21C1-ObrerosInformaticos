
struct IntegerParser {
    data: u32
}

impl IntegerParser {
    pub fn new() -> Self {
        IntegerParser { data: 0 }
    }
}

impl ProtocolParser for IntegerParser {
    fn get_prefix(&self) -> char {
        return ':';
    }

    fn feed(&mut self, line: &String) -> bool {
        let len = line.len();
        self.data = line[1..len - 2].parse().unwrap();
        return true;
    }

    fn build(&self) -> ProtocolType {
        return ProtocolType::Integer(self.data);
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_parse_integer() {
        let sample = ":54\r\n".to_string();
        let mut parser = IntegerParser::new();

        assert!(parser.feed(&sample));

        let result = parser.build().integer();
        assert_eq!(result, 54);
    }
}
