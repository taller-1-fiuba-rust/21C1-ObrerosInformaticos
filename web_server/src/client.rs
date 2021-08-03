use std::net::{TcpStream};
use std::io::{Read, Write};
use redis_protocol::types::ProtocolType;
use std::str::from_utf8;
use std::time::Duration;
use redis_protocol::parser::ParserFactory;

pub fn send_request(connection_port: String, request: &String) -> Result<Vec<u8>, &'static str> {
  
  match TcpStream::connect(connection_port) {
    Ok(mut stream) => {

        let vector: Vec<&str> = request.split(" ").collect();
        let mut protocol_vector: Vec<ProtocolType> = vec![];
        for str in vector {
          protocol_vector.push(ProtocolType::String(str.to_string()));
        }
        let protocol_string = ProtocolType::Array(protocol_vector).serialize();

        stream
            .set_read_timeout(Some(Duration::from_millis(10)))
            .ok()
            .ok_or("Failed to read from socket")?;
        stream.write(protocol_string.as_bytes()).unwrap();


        let mut buffer = [0; 512];
        let mut response_contents = Vec::new();
        loop {
             match stream.read(&mut buffer) {
                 Ok(r) => {
                     if r == 0 {
                         break;
                     }
                     response_contents.extend_from_slice(&buffer[0..r]);
                 }
                 Err(_) => {
                     if !response_contents.is_empty() {
                         break;
                     }
                 }
             }
        }

        let response_str = String::from_utf8(response_contents).unwrap();
        let mut chars = response_str.chars();
        return if let Some(mut parser) = ParserFactory::create(chars.next().unwrap()) {
            for line in response_str.split("\r\n") {
                println!("{}", line);
                if parser.feed(&format!("{}\r\n", line)).unwrap() {
                    break;
                }
            }
            Ok(parser.build().to_string().as_bytes().to_vec())
        } else {
            Err("Invalid RESP string")
        }
    },
    Err(_) => {
      Err("fail to get response")
    }
  }
}