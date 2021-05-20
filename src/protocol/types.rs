#[derive(Clone)]
pub enum ProtocolType {
    String(String),
    Integer(i32),
    Array(Vec<ProtocolType>),
}
#[allow(dead_code)]
impl ProtocolType {
    pub fn array(self) -> Vec<ProtocolType> {
        if let ProtocolType::Array(vec) = self {
            vec
        } else {
            panic!("Type is not array")
        }
    }

    pub fn integer(&self) -> i32 {
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
            ProtocolType::Array(vec) => format!(
                "*{}\r\n{}",
                vec.len(),
                vec.iter()
                    .map(|x| x.serialize())
                    .collect::<Vec<_>>()
                    .join("")
            ),
            ProtocolType::String(str) => format!("${}\r\n{}\r\n", str.len(), str),
            ProtocolType::Integer(int) => format!(":{}\r\n", int.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::protocol::types::ProtocolType;

    #[test]
    fn test_get_integer() {
        let val = ProtocolType::Integer(10);
        assert_eq!(val.integer(), 10);
    }

    #[test]
    fn test_get_negative_integer() {
        let val = ProtocolType::Integer(-10);
        assert_eq!(val.integer(), -10);
    }

    #[test]
    fn test_get_string() {
        let val = ProtocolType::String("Hi!".to_string());
        assert_eq!(val.string(), "Hi!");
    }

    #[test]
    fn test_get_array() {
        let val = ProtocolType::Array(vec![
            ProtocolType::Integer(10),
            ProtocolType::String("Hi!".to_string()),
        ]);
        let arr = val.array();
        assert_eq!(arr.len(), 2);
        assert_eq!(arr[0].integer(), 10);
        assert_eq!(arr[1].clone().string(), "Hi!");
    }

    #[test]
    fn test_serialize_integer() {
        let val = ProtocolType::Integer(10);
        assert_eq!(val.serialize(), ":10\r\n");
    }

    #[test]
    fn test_serialize_negative_integer() {
        let val = ProtocolType::Integer(-10);
        assert_eq!(val.serialize(), ":-10\r\n");
    }

    #[test]
    fn test_serialize_string() {
        let val = ProtocolType::String("Hi!".to_string());
        assert_eq!(val.serialize(), "$3\r\nHi!\r\n");
    }

    #[test]
    fn test_serialize_array() {
        let val = ProtocolType::Array(vec![
            ProtocolType::Integer(10),
            ProtocolType::String("Hi!".to_string()),
        ]);
        assert_eq!(val.serialize(), "*2\r\n:10\r\n$3\r\nHi!\r\n");
    }

    #[test]
    fn test_serialize_nested_array() {
        let val = ProtocolType::Array(vec![
            ProtocolType::Integer(10),
            ProtocolType::Array(vec![ProtocolType::Integer(4), ProtocolType::Integer(3)]),
            ProtocolType::Integer(1),
        ]);
        assert_eq!(val.serialize(), "*3\r\n:10\r\n*2\r\n:4\r\n:3\r\n:1\r\n");
    }
}
