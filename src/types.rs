#[derive(Clone)]
pub enum ProtocolType {
    String(String),
    Integer(u32),
    Array(Vec<ProtocolType>)
}

impl ProtocolType {
    pub fn array(self) -> Vec<ProtocolType> {
        if let ProtocolType::Array(a) = self {
            a
        } else {
            panic!("Type is not array")
        }
    }

    pub fn integer(&self) -> u32{
        if let ProtocolType::Integer(a) = *self {
            a
        } else {
            panic!("Type is not integer")
        }
    }

    pub fn string(self) -> String {
        if let ProtocolType::String(a) = self {
            a
        } else {
            panic!("Type is not string")
        }
    }
}

    }
}

impl ProtocolType for SimpleString {
    fn get_prefix() -> String {
        "+".to_string()
    }

    fn parse(data: String) -> Self {
        assert!(data[0] == Self::get_prefix());
        let i = 1;
        while i < data.len() && data[i] != '\r' {

        }
        SimpleString {

        }
    }
}