use std::collections::HashMap;

pub struct Response {
    status: u32,
    body: Vec<u8>,
    reason: String,
    headers: HashMap<String, String>,
}

impl Response {
    pub fn new() -> Self {
        Response {
            status: 200,
            body: Vec::new(),
            reason: "OK".to_string(),
            headers: HashMap::new(),
        }
    }

    pub fn with_status(mut self, status: u32) -> Self {
        self.status = status;
        self
    }

    #[allow(dead_code)]
    pub fn with_reason(mut self, reason: &str) -> Self {
        self.reason = reason.to_string();
        self
    }

    #[allow(dead_code)]
    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_body(mut self, body: Vec<u8>) -> Self {
        self.body = body;
        self
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut headers = self.headers.clone();
        let body = &self.body;
        if !body.is_empty() {
            headers.insert("Content-Length".to_string(), body.len().to_string());
        }

        let status_line = format!("HTTP/1.1 {} {}", self.status, self.reason);
        let headers_str = headers
            .iter()
            .map(|x| format!("{}: {}", x.0.clone(), x.1.clone()))
            .collect::<Vec<String>>()
            .join("\r\n");

        let partial_res = format!("{}\r\n{}\r\n\r\n", status_line, headers_str);
        let mut res = Vec::new();
        res.extend_from_slice(partial_res.as_bytes());
        res.extend_from_slice(body);
        res.extend_from_slice("\r\n".as_bytes());
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_code() {
        let response_str = String::from_utf8(Response::new().with_status(500).serialize()).unwrap();
        assert_eq!(response_str, "HTTP/1.1 500 OK\r\n\r\n\r\n\r\n");
    }

    #[test]
    fn test_response_reason() {
        let response_str = String::from_utf8(Response::new().with_reason("NOT OK").serialize()).unwrap();
        assert_eq!(response_str, "HTTP/1.1 200 NOT OK\r\n\r\n\r\n\r\n");
    }

    #[test]
    fn test_response_body() {
        let response_str = String::from_utf8(Response::new().with_body("This is a test body".as_bytes().to_owned()).serialize()).unwrap();
        assert_eq!(response_str, "HTTP/1.1 200 OK\r\nContent-Length: 19\r\n\r\nThis is a test body\r\n");
    }

    #[test]
    fn test_response_header() {
        let response_str = String::from_utf8(Response::new().with_header("Good", "luck").serialize()).unwrap();
        assert_eq!(response_str, "HTTP/1.1 200 OK\r\nGood: luck\r\n\r\n\r\n");
    }
}
