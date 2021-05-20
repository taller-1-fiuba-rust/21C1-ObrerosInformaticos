pub mod array;
pub mod integer;

pub trait ProtocolParser {
    fn get_prefix(&self) -> char;
    fn feed(&mut self, line: &String) -> bool;
    fn build(&self) -> ProtocolType;
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