pub mod array;
pub mod error;
pub mod integer;
pub mod string;

use crate::parser::array::*;
use crate::parser::error::*;
use crate::parser::integer::*;
use crate::parser::string::*;
use crate::types::ProtocolType;

///
/// Common functions between all RESP parsers
///
pub trait ProtocolParser {
    /// Return the prefix of this RESP parser
    fn get_prefix(&self) -> char;
    /// Process a new line. Returns true if it finished parsing.
    fn feed(&mut self, line: &str) -> Result<bool, String>;
    /// Build the parsed ProtocolType
    fn build(&self) -> ProtocolType;
}

///
/// Parses a serialized RESP array into a Vec<ProtocolType>
///
pub struct ParserFactory;

impl ParserFactory {
    /// Create a new parser from a RESP prefix/symbol (-, +, *, :, $)
    pub fn create(symbol: char) -> Option<Box<dyn ProtocolParser>> {
        let options: Vec<Box<dyn ProtocolParser>> = vec![
            Box::new(IntegerParser::new()),
            Box::new(SimpleStringParser::new()),
            Box::new(ArrayParser::new()),
            Box::new(BulkStringParser::new()),
            Box::new(ErrorParser::new()),
        ];
        for option in options {
            if option.get_prefix() == symbol {
                return Some(option);
            }
        }
        None
    }

    /// Return a bool representing if the given symbol has a valid parser.
    fn has_symbol(symbol: char) -> bool {
        return Self::create(symbol).is_some();
    }
}
