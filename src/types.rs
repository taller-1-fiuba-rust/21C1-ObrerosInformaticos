enum ProtocolType {
    String(String),
    Integer(u32),
    Array(Vec<ProtocolType>),
    Mixed(MixedType)
}

impl SimpleString {
    pub fn new(data: String) -> Self {

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