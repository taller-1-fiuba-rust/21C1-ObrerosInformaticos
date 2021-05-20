#[derive(Clone)]
pub enum ProtocolType {
    String(String),
    Integer(u32),
    Array(Vec<ProtocolType>)
}

impl ProtocolType {
    pub fn array(self) -> Vec<ProtocolType> {
        if let ProtocolType::Array(vec) = self {
            vec
        } else {
            panic!("Type is not array")
        }
    }

    pub fn integer(&self) -> u32{
        if let ProtocolType::Integer(int) = *self {
            int
        } else {
            panic!("Type is not integer")
        }
    }

    pub fn string(self) -> String {
        if let ProtocolType::String(str) = self {
            str
        } else {
            panic!("Type is not string")
        }
    }
}

pub trait ProtocolParser {
    fn get_prefix(&self) -> char;
    fn feed(&mut self, line: &String) -> bool;
    fn build(&self) -> ProtocolType;
}

struct SimpleStringParser {
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

struct BulkStringParser {
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

struct ParserFactory;

impl ParserFactory {
    fn create(symbol: char) -> Option<Box<dyn ProtocolParser>> {
        let options: Vec<Box<dyn ProtocolParser>> = vec![
                Box::new(IntegerParser::new()),
                Box::new(SimpleStringParser::new()),
                Box::new(ArrayParser::new()),
                Box::new(BulkStringParser::new()),
            ];
        for option in options {
            if option.get_prefix() == symbol {
                return Some(option);
            }
        }
        None
    }

    fn has_symbol(symbol: char) -> bool {
        return Self::create(symbol).is_some();
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

    #[test]
    fn test_parse_error() {

    }

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