use std::collections::HashMap;
use crate::http::method::Method;

pub struct Request {
    method: Method,
    endpoint: String,
    headers: HashMap<String, String>,
    body:
}

impl Request {
    pub fn try_parse(request_str: String) -> Result<Request, &'static str> {
        let mut lines = request_str.split("\n");

        let mut headers = Request::parse_headers();


        Request {
            method,
            endpoint,
            headers,
            body
        }
    }

    fn parse_headers(&mut Split<&str>) -> Result<HashMap<String, String>, &'static str> {
        let mut headers = HashMap::new();

        loop {
            match lines.next() {
                Ok(l) => {
                    if l == "" {
                        break;
                    }
                    let pair = l.split(":").collect::<Vec<String>>();
                    if pair.len() != 2 {
                        return Err("Malformed HTTP headers")
                    }
                    headers.insert(pair[0].trim(), pair[1].trim());
                }
                None => return Err("Malformed HTTP headers")
            }
        }

        Ok(headers)
    }
}