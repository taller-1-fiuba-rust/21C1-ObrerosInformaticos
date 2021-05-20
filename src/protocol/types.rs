#[derive(Clone)]
pub enum ProtocolType {
    String(String),
    Integer(u32),
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

    pub fn integer(&self) -> u32{
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
}

#[cfg(test)]
mod tests {

}