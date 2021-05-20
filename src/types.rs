#[derive(Clone)]
pub enum ProtocolType {
    String(String),
    Integer(u32),
    Array(Vec<ProtocolType>)
}

impl ProtocolType {
    pub fn array(self) -> Vec<ProtocolType> {
        if let ProtocolType::Array(a) = self {
            a
        } else {
            panic!("Type is not array")
        }
    }

    pub fn integer(&self) -> u32{
        if let ProtocolType::Integer(a) = *self {
            a
        } else {
            panic!("Type is not integer")
        }
    }

    pub fn string(self) -> String {
        if let ProtocolType::String(a) = self {
            a
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

struct ParserFactory;

impl ParserFactory {
    fn create(symbol: char) -> Option<Box<dyn ProtocolParser>> {
        let options: Vec<Box<dyn ProtocolParser>> = vec![
                Box::new(IntegerParser::new()),
                Box::new(SimpleStringParser::new()),
                Box::new(ArrayParser::new()),
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
}