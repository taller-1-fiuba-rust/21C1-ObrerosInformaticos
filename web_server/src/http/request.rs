use crate::http::method::Method;
use std::collections::HashMap;
use std::str::Split;

pub struct Request {
    method: Method,
    endpoint: String,
    headers: HashMap<String, String>,
    body: String,
}

impl Request {
    pub fn parse(request_str: String) -> Result<Request, &'static str> {
        #[allow(clippy::single_char_pattern)]
        let mut lines = request_str.split("\n");

        let (method, endpoint) = Request::parse_method_and_endpoint(&mut lines)?;
        let headers = Request::parse_headers(&mut lines)?;

        let mut body = String::new();
        if headers.contains_key("Content-Length") {
            let length = headers["Content-Length"]
                .parse::<u32>()
                .ok()
                .ok_or("Invalid Content-Length header")?;
            body = lines.collect::<Vec<&str>>().join("\n")[..length as usize].to_string();
        }

        Ok(Request {
            method,
            endpoint,
            headers,
            body,
        })
    }

    fn parse_method_and_endpoint(
        lines: &mut Split<&str>,
    ) -> Result<(Method, String), &'static str> {
        match lines.next() {
            Some(l) => {
                let parts = l.split(' ').collect::<Vec<&str>>();
                if parts.len() != 3 {
                    return Err("Malformed HTTP");
                }

                let method = Method::parse(parts[0])?;
                Ok((method, parts[1].to_string()))
            }
            None => Err("Malformed HTTP request"),
        }
    }

    fn parse_headers(lines: &mut Split<&str>) -> Result<HashMap<String, String>, &'static str> {
        let mut headers = HashMap::new();

        loop {
            match lines.next() {
                Some(l) => {
                    let maybe_idx = l.find(':');
                    if maybe_idx.is_none() {
                        break;
                    }
                    let idx = maybe_idx.unwrap();
                    headers.insert(
                        l[..idx].trim().to_string(),
                        l[(idx + 1_usize)..].trim().to_string(),
                    );
                }
                None => return Err("Malformed HTTP headers, none"),
            }
        }

        Ok(headers)
    }
}

impl ToString for Request {
    fn to_string(&self) -> String {
        format!(
            "{} {}\n{}\n{}",
            self.method.to_string(),
            self.endpoint,
            self.headers
                .iter()
                .map(|x| format!("{}: {}", x.0.clone(), x.1.clone()))
                .collect::<Vec<String>>()
                .join("\n"),
            self.body
        )
    }
}
