use crate::protocol::parser::ProtocolParser;
use crate::protocol::types::ProtocolType;
use crate::protocol::parser::ParserFactory;

pub struct ArrayParser {
    count: u32,
    parsed_header: bool,
    last_parser_completed: bool,
    parsers: Vec<Box<dyn ProtocolParser>>,
}

impl ArrayParser {
    pub fn new() -> Self {
        ArrayParser {
            count: 0,
            parsed_header: false,
            last_parser_completed: true,
            parsers: Vec::new()
        }
    }
}

impl ProtocolParser for ArrayParser {
    fn get_prefix(&self) -> char {
        return '*';
    }

    fn feed(&mut self, line: &String) -> bool {
        let symbol = line.chars().nth(0).unwrap();
        if ParserFactory::has_symbol(symbol) {
            if symbol == self.get_prefix() && !self.parsed_header {
                let len = line.len();
                self.count = line[1..len - 2].parse().unwrap();
                self.parsed_header = true;
                return false;
            } else {
                if self.last_parser_completed {
                    let parser = ParserFactory::create(symbol);
                    self.parsers.push(parser.unwrap());
                }
            }
        }
        let len = self.parsers.len();
        self.last_parser_completed = self.parsers[len - 1].feed(line);
        self.last_parser_completed && len == self.count as usize
    }

    fn build(&self) -> ProtocolType {
        assert_eq!(self.count, self.parsers.len() as u32);

        let mut data = Vec::new();
        for parser in &self.parsers {
            data.push(parser.build());
        }
        return ProtocolType::Array(data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_array(lines: Vec<&str>) -> Vec<ProtocolType> {
        let mut parser = ArrayParser::new();
        for line in lines {
            parser.feed(&line.to_string());
        }
        parser.build().array()
    }

    #[test]
    fn test_parse_array() {
        let lines = vec!["*2\r\n", ":3\r\n", ":42\r\n"];
        let result = parse_array(lines);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].integer(), 3);
        assert_eq!(result[1].integer(), 42);
    }

    #[test]
    fn test_parse_mixed_array() {
        let lines = vec!["*2\r\n", ":3\r\n", "+OK\r\n"];
        let result = parse_array(lines);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].integer(), 3);
        assert_eq!(result[1].clone().string(), "OK");
    }

    #[test]
    fn parse_nested_array() {
        let lines = vec!["*2\r\n", ":2\r\n", "*1\r\n", ":4\r\n"];
        let result = parse_array(lines);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].integer(), 2);
        let nested_arr = result[1].clone().array();
        assert_eq!(nested_arr[0].integer(), 4);
    }

    #[test]
    fn parse_empty_array() {
        let lines = vec!["*0\r\n"];
        let result = parse_array(lines);

        assert_eq!(result.len(), 0);
    }

    fn split_lines(string: &str) -> Vec<&str> {
        let mut vector: Vec<&str> = Vec::new();
        let chars: Vec<char> = string.chars().collect();
        let mut i = 0;
        let mut offset = 0;
        let len = chars.len();
        while i < len {
            if chars[i] == '\n' {
                vector.push(&string[offset..i+1]);
                offset = i+1;
            }
            i += 1;
        }
        vector
    }

    #[test]
    fn parse_array_with_bulk_strings() {
        let lines = split_lines("*2\r\n$3\r\nfoo\r\n$3\r\nbar\r\n");
        let result = parse_array(lines);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].clone().string(), "foo");
        assert_eq!(result[1].clone().string(), "bar");
    }
}