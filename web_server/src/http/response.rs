use std::collections::HashMap;

pub struct Response {
    status: u32,
    body: String,
    reason: String,
    headers: HashMap<String, String>
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

    pub fn with_reason(mut self, reason: &String) -> Self {
        self.reason = reason.clone();
        self
    }

    pub fn with_header(mut self, key: &String, value: &String) -> Self {
        self.headers.insert(key.clone(), value.clone());
        self
    }

    pub fn with_body(mut self, body: &String) -> Self {
        self.body = body.clone();
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
        let headers_str = headers.iter()
            .map(|x| format!("{}: {}", x.0.clone(), x.1.clone()))
            .collect::<Vec<String>>()
            .join("\r\n");
        format!("{}\r\n{}\r\n\r\n{}\r\n", status_line, headers_str, body)
    }
}


