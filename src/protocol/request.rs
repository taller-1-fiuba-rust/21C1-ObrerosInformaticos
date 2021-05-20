use crate::protocol::parser::array::ArrayParser;
use crate::protocol::parser::ProtocolParser;
use crate::protocol::command::Command;

pub struct Request {
    parser: ArrayParser
}

impl Request {
    pub fn new() -> Self {
        Request {
            parser: ArrayParser::new()
        }
    }

    pub fn feed(&mut self, line: &String) {
        self.parser.feed(line);
    }

    pub fn build(&self) -> Command {
        let mut types = self.parser.build().array();
        let symbol = types[0].clone().string();
        types.remove(0);
        Command::new(symbol, types)
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn parse_simple_request() {
        /*
        let sample_request = "*2\r\n$4\r\nLLEN\r\n$6\r\nmylist\r\n".to_string();
        let lines = sample_request.split("\r\n");
        let mut request = Request::new();

        for line in lines {
            request.feed(&line);
        }

        let command = request.build();*/
        //assert_eq!(command);
    }
}