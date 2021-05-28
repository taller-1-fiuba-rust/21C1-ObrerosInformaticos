#[derive(Clone)]
pub enum ProtocolType {
    String(String),
    Integer(i32),
    Array(Vec<ProtocolType>),
    Error(String),
}

#[allow(dead_code)]
impl ProtocolType {
    pub fn array(self) -> Result<Vec<ProtocolType>, &'static str> {
        match self {
            ProtocolType::Array(vec) => Ok(vec),
            _ => Err("Type is not array"),
        }
    }

    pub fn integer(&self) -> Result<i32, &'static str> {
        match self {
            ProtocolType::Integer(int) => Ok(*int),
            ProtocolType::String(str_int) => match str_int.parse() {
                Ok(i) => Ok(i),
                Err(_) => Err("Failed to cast string"),
            },
            _ => Err("Type is not integer"),
        }
    }

    pub fn string(self) -> Result<String, &'static str> {
        match self {
            ProtocolType::String(str) => Ok(str),
            _ => Err("Type is not string"),
        }
    }

    pub fn error(self) -> Result<String, &'static str> {
        match self {
            ProtocolType::Error(str) => Ok(str),
            _ => Err("Type is not error"),
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
            ProtocolType::Error(err) => format!("-{}\r\n", err),
        }
    }
}

impl ToString for ProtocolType {
    fn to_string(&self) -> String {
        match self {
            ProtocolType::Array(vec) => format!(
                "[{}]",
                vec.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            ProtocolType::String(str) => str.clone(),
            ProtocolType::Integer(int) => int.to_string(),
            ProtocolType::Error(err) => err.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::protocol::types::ProtocolType;

    #[test]
    fn test_get_integer() {
        let val = ProtocolType::Integer(10);
        assert_eq!(val.integer().unwrap(), 10);
    }

    #[test]
    fn test_get_string_integer() {
        let val = ProtocolType::String("10".to_string());
        assert_eq!(val.integer().unwrap(), 10);
    }

    #[test]
    fn test_get_string_negative_integer() {
        let val = ProtocolType::String("-10".to_string());
        assert_eq!(val.integer().unwrap(), -10);
    }

    #[test]
    fn test_get_negative_integer() {
        let val = ProtocolType::Integer(-10);
        assert_eq!(val.integer().unwrap(), -10);
    }

    #[test]
    fn test_get_string() {
        let val = ProtocolType::String("Hi!".to_string());
        assert_eq!(val.string().unwrap(), "Hi!");
    }

    #[test]
    fn test_get_array() {
        let val = ProtocolType::Array(vec![
            ProtocolType::Integer(10),
            ProtocolType::String("Hi!".to_string()),
        ]);
        let arr = val.array().unwrap();
        assert_eq!(arr.len(), 2);
        assert_eq!(arr[0].integer().unwrap(), 10);
        assert_eq!(arr[1].clone().string().unwrap(), "Hi!");
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
