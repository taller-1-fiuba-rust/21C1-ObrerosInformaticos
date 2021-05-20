#[derive(Clone)]
pub enum ProtocolType {
    String(String),
    Integer(i32),
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

    pub fn integer(&self) -> i32{
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

    pub fn serialize(&self) -> String {
        match self {
            ProtocolType::Array(vec) =>
                format!("*{}\r\n{}", vec.len(), vec.iter().map(|x| x.serialize()).collect::<Vec<_>>().join("")),
            ProtocolType::String(str) =>
                format!("${}\r\n{}\r\n", str.len(), str),
            ProtocolType::Integer(int) =>
                format!(":{}\r\n", int.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {

}