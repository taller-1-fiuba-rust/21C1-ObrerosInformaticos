use crate::http::method::Method;
use core::str::SplitInclusive;
use std::collections::HashMap;

pub struct Request<'a> {
    method: Method,
    endpoint: &'a str,
    headers: HashMap<&'a str, &'a str>,
    body: &'a str,
}

impl<'a> Request<'a> {
    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn endpoint(&self) -> &str {
        self.endpoint
    }

    pub fn body(&self) -> &str {
        &self.body
    }

    pub fn parse(request_str: &'a str) -> Result<Request, &'static str> {
        let mut lines = request_str.split_inclusive("\r\n");
        let mut offset = 0;
        let (method, endpoint) = Request::parse_method_and_endpoint(&mut lines, &mut offset)?;
        let headers = Request::parse_headers(&mut lines, &mut offset)?;

        let mut body = "";
        if headers.contains_key("Content-Length") {
            let length = headers["Content-Length"]
                .parse::<u32>()
                .ok()
                .ok_or("Invalid Content-Length header")?;
            body = &request_str[offset as usize..(offset + length) as usize];
        }

        Ok(Request {
            method,
            endpoint,
            headers,
            body,
        })
    }

    #[allow(dead_code)]
    pub fn headers(&self) -> &HashMap<&str, &str> {
        &self.headers
    }

    fn parse_method_and_endpoint(
        lines: &mut SplitInclusive<'a, &'a str>,
        offset: &mut u32,
    ) -> Result<(Method, &'a str), &'static str> {
        match lines.next() {
            Some(l) => {
                *offset += l.len() as u32;
                let parts = l.split(' ').collect::<Vec<&str>>();
                if parts.len() != 3 {
                    return Err("Malformed HTTP");
                }

                let method = Method::parse(parts[0])?;
                Ok((method, parts[1]))
            }
            None => Err("Malformed HTTP request"),
        }
    }

    fn parse_headers(
        lines: &mut SplitInclusive<'a, &'a str>,
        offset: &mut u32,
    ) -> Result<HashMap<&'a str, &'a str>, &'static str> {
        let mut headers = HashMap::new();

        loop {
            match lines.next() {
                Some(l) => {
                    *offset += l.len() as u32;
                    let maybe_idx = l.find(':');
                    if maybe_idx.is_none() {
                        break;
                    }
                    let idx = maybe_idx.unwrap();
                    headers.insert(l[..idx].trim(), l[(idx + 1_usize)..].trim());
                }
                None => return Err("Malformed HTTP headers, none"),
            }
        }

        Ok(headers)
    }
}

impl ToString for Request<'_> {
    fn to_string(&self) -> String {
        let mut headers = self
            .headers
            .iter()
            .map(|x| format!("{}: {}", x.0.to_owned(), x.1.to_owned()))
            .collect::<Vec<String>>();
        headers.sort();
        format!(
            "{} {}\n{}\n{}",
            self.method.to_string(),
            self.endpoint,
            headers.join("\n"),
            self.body
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let mut headers = HashMap::new();
        headers.insert("Key", "Value");
        headers.insert("Test", "Header");
        headers.insert("Content-Length", "9");
        let request = Request::parse("POST /test HTTP/1.1\r\nContent-Length: 9\r\nKey: Value\r\nTest: Header\r\n\r\nTest body").unwrap();
        assert_eq!(
            request.to_string(),
            Request {
                method: Method::Post,
                endpoint: "/test",
                headers,
                body: "Test body"
            }
            .to_string()
        );
    }
}
