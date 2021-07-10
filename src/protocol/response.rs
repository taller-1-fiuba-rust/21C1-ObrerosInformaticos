use crate::protocol::types::ProtocolType;
use std::fmt;

///
/// Struct for building responses in RESP format. Stores ProtocolTypes and serializes them.
///
pub struct ResponseBuilder {
    results: Vec<ProtocolType>,
}

impl ResponseBuilder {
    /// Create a new ResponseBuilder
    pub fn new() -> Self {
        ResponseBuilder {
            results: Vec::new(),
        }
    }

    /// Adds a new value into the RESP response
    pub fn add(&mut self, val: ProtocolType) {
        self.results.push(val);
    }

    /// Returns the response as a String
    pub fn to_string(&self) -> String {
        self.results
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("")
    }

    /// Serialiazes the objects into a RESP compatible format.
    pub fn serialize(&self) -> String {
        self.results
            .iter()
            .map(|x| x.serialize())
            .collect::<Vec<String>>()
            .join("")
    }

    pub fn is_empty(&self) -> bool {
        self.results.is_empty()
    }
}

impl fmt::Display for ResponseBuilder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "I am A")
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        let mut response = ResponseBuilder::new();
        response.add(ProtocolType::String("Test 1!".to_string()));
        response.add(ProtocolType::Array(vec![
            ProtocolType::Integer(1),
            ProtocolType::Integer(2),
            ProtocolType::Array(vec![
                ProtocolType::Integer(2),
                ProtocolType::String("Test 2!".to_string()),
            ]),
        ]));
        response.add(ProtocolType::Integer(1));
        response.add(ProtocolType::Integer(-15));
        assert_eq!(
            response.serialize(),
            "$7\r\nTest 1!\r\n*3\r\n:1\r\n:2\r\n*2\r\n:2\r\n$7\r\nTest 2!\r\n:1\r\n:-15\r\n"
        )
    }
}
