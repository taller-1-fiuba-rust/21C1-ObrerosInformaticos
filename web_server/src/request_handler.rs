use crate::http::request::Request;
use crate::http::response::Response;
use crate::client;
use std::str::from_utf8;
use std::fs::File;
use std::io::Read;
use redis_protocol::types::ProtocolType;

pub struct RequestHandler {
    port: u16,
}

impl RequestHandler {
    pub fn new(port: u16) -> Self {
        RequestHandler {
            port,
        }
    }

    pub fn handle(&self, request: &Request) -> Response {

        let connection_port = "127.0.0.1:".to_string() + &self.port.to_string();
        let method = request.method().to_string();

        if method == "GET".to_string() {
            let mut file_path = "/front-end".to_string() + request.endpoint(); 
            if file_path.chars().last().unwrap() == '/' {
                file_path += "index.html";
            }
            let file_content = read_lines(&file_path);
            match file_content {
                Ok(content) => {
                    Response::new().with_status(200).with_body(&from_utf8(&content).unwrap().to_string())
                }
                Err(_) => Response::new().with_status(404).with_body(&"Not found".to_string())
            }
        } else if method == "POST".to_string(){
            if request.endpoint() == "/eval" && valid_command(request.body().clone()) {
                let response = client::send_request(connection_port, request.body());
                match response {
                    Ok(resp) => {
                        Response::new().with_status(200).with_body(&resp)
                    },
                    Err(_) => {
                        Response::new().with_status(404).with_body(&ProtocolType::String("fail to get response".to_string()).to_string())
                    }
                }
            } else {
                Response::new().with_status(404).with_body(&ProtocolType::String("Request not correct".to_string()).to_string())
            }
        }else {
            Response::new().with_status(404).with_body(&ProtocolType::String("Request not correct".to_string()).to_string())
        }

    }
}

pub fn valid_command(body: String) -> bool {
    let commands = ["unsubscribe", "subscribe", "publish", "punsubscribe", "pubsub", "monitor", "quit"];
    let cmd: String = body
    .split_whitespace()
    .next()
    .unwrap_or("")
    .to_string();
    !commands.iter().any(|e| *e == cmd)
}

pub fn read_lines(filename: &str) -> Result<Vec<u8>, &'static str> {
    let file = File::open(filename);
    match file {
        Ok(mut f) => {
            let mut contents = vec![];
            f.read_to_end(&mut contents).expect("Unable to read to bytes");
            Ok(contents)
        }
        Err(_i) => Err("Not existing file"), 
    }
}
