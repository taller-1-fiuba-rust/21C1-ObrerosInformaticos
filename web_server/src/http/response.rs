use std::collections::HashMap;

pub struct Response {
    status: u32,
    body: String,
    reason: String,
    headers: HashMap<String, String>,
}

impl Response {
    pub fn new() -> Self {
        Response {
            status: 200,
            body: String::new(),
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

    pub fn with_body(mut self, body: &str) -> Self {
        self.body = body.to_string();
        self
    }

    pub fn serialize(&self) -> String {
        let mut headers = self.headers.clone();
        let body = &self.body;
        if !body.is_empty() {
            headers.insert("Content-Length".to_string(), body.len().to_string());
        }
        headers.insert("hola".to_string(), "taller".to_string());
        let status_line = format!("HTTP/1.1 {} {}", self.status, self.reason);
        let headers_str = headers
            .iter()
            .map(|x| format!("{}: {}", x.0.clone(), x.1.clone()))
            .collect::<Vec<String>>()
            .join("\r\n");
        format!("{}\r\n{}\r\n\r\n{}\r\n", status_line, headers_str, body)
    }
}
