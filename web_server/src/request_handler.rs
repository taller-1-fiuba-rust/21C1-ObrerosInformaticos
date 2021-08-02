use crate::http::request::Request;
use crate::http::response::Response;
use crate::client;

const GET: String = "GET".to_string();
const POST: String = "POST".to_string();

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
        
        match request.method().to_string() {
            GET => {
                //Archivo del front
            },
            POST => {
                if request.endpoint() == "/eval" && valid_command(request.body().clone()) {
                    let response = client::send_request(connection_port, request.body());
                    match response {
                        Ok(resp) => {
                            Response::new().with_status(200).with_body(&resp)
                        },
                        Err(e) => {
                            Response::new().with_status(404).with_body(e)
                        }
                    }
                } else {
                    Response::new().with_status(404).with_body(&"Resquest not correct".to_string())
                }
            },
        }
    }
}

pub fn valid_command(body: String) -> bool {
    let commands = ["UNSUBSCRIBE", "SUBSCRIBE", "PUBLISH", "PUNSUBSCRIBE", "PUBSUB", "MONITOR", "QUIT"];
    let cmd: String = body
    .split_whitespace()
    .next()
    .unwrap_or("")
    .to_string();
    !commands.iter().any(|e| *e == cmd)
}
