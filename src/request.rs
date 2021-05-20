struct Request {
    types: Vec<ProtocolType>,
    length: i32,
    operators: Vec<ProtocolParser>,
    operands: Vec<ParserType>,
}

impl Request {
    pub fn new() -> Self {
        Request {
            length: 0,
            types: Vec::new(),
            parser_stack: Vec::new()
        }
    }

    pub fn feed(&mut self, line: &String) {
        if self::has_prefix(line) {
            self.parser_stack.push(self::select_parser(line));
        }
        if self.parser_stack.last().feed(line) {
            let parser = self.parser_stack.pop();
            self.types.push(parser.build());
        }
    }

    pub fn build(&self) -> Command {

        if self.parser_stack.len() > 0
    }

    fn has_prefix(line: &String) -> bool {
        return line[0]asdasd;
    }

    fn select_parser(line: &String) -> ProtocolParser {
        match line[0] {

        }
    }
}

enum ParserType {
    Raw(String),
    Type(ProtocolType)
}

trait ProtocolParser {
    fn get_prefix() -> String;
    fn feed(&mut self, line: &String) -> bool;
    fn build(&self) -> ProtocolType;
}

struct SimpleStringParser {
    data: String
}

impl ProtocolParser for SimpleStringParser {
    fn get_prefix() -> String {
        return '+';
    }

    fn feed(&mut self, line: &String) -> bool {
        let l = line.len();
        self.data = line[1..l-2];
        return true;
    }

    fn build(&self) -> ProtocolType {
        return ProtocolType::String(self.data);
    }
}

struct ArrayParser {
    count: u32,
    parsers: Vec<ProtocolParser>,
}

impl ProtocolParser for ArrayParser {
    fn get_prefix() -> String {
        return '*';
    }

    fn feed(&mut self, line: &String) -> bool {
        let symbol = line[0];
        if symbol {
            return if symbol == self::get_prefix() {
                let len = line.len();
                self.count = line[1..len - 2].parse().unwrap();
                false
            } else {
                let parser = select_parser(symbol);
                self.parsers.push(parser);
            }
        }
        self.parsers[len - 1].feed(line);
        return false;
    }

    fn build(&self) -> ProtocolType {
        assert_eq!(self.count, parsers.len());
        let mut data = Vec::new();
        for parser in parsers {
            data.push(parser.build());
        }
        return ProtocolType::Array(data);
    }
}


#[cfg(tests)]
mod tests {

    #[test]
    fn parse_array() {
        let lines = vec!["*2\r\n", ":3\r\n", ":42\r\n"];
        let array_parser = ArrayParser::new();
        for line in lines {
            array_parser.feed(line.to_string());
        }
        let result = array_parser.build();
        assert!(result.Array.is_some());
        assert_eq!(result.Array.len(), 2);
        assert_eq!(result.Array[0], 3);
        assert_eq!(result.Array[1], 42);
    }

    #[test]
    fn parse_nested_array() {

    }

    #[test]
    fn parse_simple_string() {
        //let sample_request = "+OK\r\n".to_string();
        //let request = Request::new(sample_request);
    }
}