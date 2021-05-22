use crate::protocol::types::ProtocolType;

pub struct ResponseBuilder {
    results: Vec<ProtocolType>,
}

impl ResponseBuilder {
    pub fn new() -> Self {
        ResponseBuilder {
            results: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn add(&mut self, val: ProtocolType) {
        self.results.push(val);
    }

    pub fn serialize(&self) -> String {
        ProtocolType::Array(self.results.clone()).serialize()
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
            "*4\r\n$7\r\nTest 1!\r\n*3\r\n:1\r\n:2\r\n*2\r\n:2\r\n$7\r\nTest 2!\r\n:1\r\n:-15\r\n"
        )
    }
}
