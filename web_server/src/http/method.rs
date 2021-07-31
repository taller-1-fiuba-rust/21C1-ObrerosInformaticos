
pub enum Method {
    Get,
    Post,
}

impl Method {
    pub fn parse(method_str: &str) -> Result<Method, &'static str> {
        match method_str {
            "GET" => Ok(Method::Get),
            "POST" => Ok(Method::Post),
            _ => Err("Not implemented")
        }
    }
}

impl ToString for Method {
    fn to_string(&self) -> String {
        match self {
            Method::Get => "GET",
            Method::Post => "POST",
        }.to_string()
    }
}