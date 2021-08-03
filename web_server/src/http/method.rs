pub enum Method {
    Get,
    Post,
}

impl Method {
    pub fn parse(method_str: &str) -> Result<Method, &'static str> {
        match method_str {
            "GET" => Ok(Method::Get),
            "POST" => Ok(Method::Post),
            _ => {
                println!("Method str es {}", method_str);
                Err("Not implemented")
            }
        }
    }
}

impl ToString for Method {
    fn to_string(&self) -> String {
        match self {
            Method::Get => "GET",
            Method::Post => "POST",
        }
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_get() {
        assert_eq!(Method::Get.to_string(), Method::parse("GET").unwrap().to_string());
    }

    #[test]
    fn test_parse_post() {
        assert_eq!(Method::Post.to_string(), Method::parse("POST").unwrap().to_string());
    }
}
