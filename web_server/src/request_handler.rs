use crate::client;
use crate::http::method::Method;
use crate::http::request::Request;
use crate::http::response::Response;
use std::fs::File;
use std::io::Read;

pub struct RequestHandler {
    port: u16,
}

const INVALID_COMMAND_MSG: &str = "I'm sorry, I don't recognize that command.";

impl RequestHandler {
    pub fn new(port: u16) -> Self {
        RequestHandler { port }
    }

    pub fn handle(&self, request: &Request) -> Response {
        match request.method() {
            Method::Get => Self::handle_resource_request(request),
            Method::Post => self.handle_eval_request(request),
        }
    }

    fn handle_resource_request(request: &Request) -> Response {
        let mut file_path = "web_server/front-end".to_string() + request.endpoint();
        if file_path.ends_with('/') {
            file_path += "index.html";
        }
        let file_contents = Self::read_lines(&file_path);
        match file_contents {
            Ok(content) => Response::new().with_status(200).with_body(content),
            Err(_) => Response::new()
                .with_status(404)
                .with_body("Not found".as_bytes().to_owned()),
        }
    }

    fn handle_eval_request(&self, request: &Request) -> Response {
        let connection_port = "127.0.0.1:".to_string() + &self.port.to_string();

        if request.endpoint() == "/eval" {
            if Self::valid_command(request.body()) {
                let body = request.body();
                let response = client::send_request(connection_port, body);
                match response {
                    Ok(resp) => Response::new().with_status(200).with_body(resp),
                    Err(_) => Response::new()
                        .with_status(500)
                        .with_body("Internal server error".as_bytes().to_owned()),
                }
            } else {
                Response::new()
                    .with_status(200)
                    .with_body(INVALID_COMMAND_MSG.as_bytes().to_owned())
            }
        } else {
            Response::new()
                .with_status(404)
                .with_body("Not Found".as_bytes().to_owned())
        }
    }

    fn valid_command(body: &str) -> bool {
        let commands = [
            "unsubscribe",
            "subscribe",
            "publish",
            "punsubscribe",
            "pubsub",
            "monitor",
            "quit",
        ];
        let cmd: &str = body.split_whitespace().next().unwrap_or("");
        !commands.iter().any(|e| *e == cmd)
    }

    fn read_lines(filename: &str) -> Result<Vec<u8>, &'static str> {
        let file = File::open(filename);
        match file {
            Ok(mut f) => {
                let mut contents: Vec<u8> = vec![];
                f.read_to_end(&mut contents).expect("Unable to read bytes");
                Ok(contents)
            }
            Err(_i) => Err("No existing file"),
        }
    }
}
