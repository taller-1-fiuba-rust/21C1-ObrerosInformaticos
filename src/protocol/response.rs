use crate::protocol::types::ProtocolType;

struct Response {
    results: Vec<ProtocolType>
}

impl Response {
    pub fn new() -> Self {
        Response {
            results: Vec::new()
        }
    }

    pub fn serialize(&self) -> String {
        let mut vec: Vec<String> = Vec::new();
        for primitive in &self.results {
            vec.push(primitive.serialize());
        }
        vec.join("")
    }
}