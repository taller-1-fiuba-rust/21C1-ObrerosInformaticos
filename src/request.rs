use crate::types::ArrayParser;
use crate::types::ProtocolParser;
use crate::command::Command;

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
    use super::*;

    #[test]
    fn parse_simple_request() {
        //let sample_request = "+OK\r\n".to_string();
        //let request = Request::new(sample_request);
    }
}