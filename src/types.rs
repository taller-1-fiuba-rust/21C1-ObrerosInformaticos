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

pub trait ProtocolParser {
    fn get_prefix(&self) -> char;
    fn feed(&mut self, line: &String) -> bool;
    fn build(&self) -> ProtocolType;
}

struct SimpleStringParser {
    data: String
}

impl SimpleStringParser {
    pub fn new() -> Self {
        SimpleStringParser { data: String::new() }
    }
}

impl ProtocolParser for SimpleStringParser {
    fn get_prefix(&self) -> char {
        return '+';
    }

    fn feed(&mut self, line: &String) -> bool {
        let l = line.len();
        self.data = line[1..l-2].to_string();
        return true;
    }

    fn build(&self) -> ProtocolType {
        return ProtocolType::String(self.data.clone());
    }
}

struct IntegerParser {
    data: u32
}

impl IntegerParser {
    pub fn new() -> Self {
        IntegerParser { data: 0 }
    }
}

impl ProtocolParser for IntegerParser {
    fn get_prefix(&self) -> char {
        return ':';
    }

    fn feed(&mut self, line: &String) -> bool {
        let len = line.len();
        self.data = line[1..len - 2].parse().unwrap();
        return true;
    }

    fn build(&self) -> ProtocolType {
        return ProtocolType::Integer(self.data);
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