use std::net::{TcpStream};
use std::io::{Read, Write};
use redis_protocol::types::ProtocolType;
use std::str::from_utf8;

pub fn send_request(connection_port: String, request: &String) -> Result<String, &'static str> {
  
  match TcpStream::connect(connection_port) {
    Ok(mut stream) => {
        let vector: Vec<&str> = request.split(" ").collect();
        let mut protocol_vector: Vec<ProtocolType> = vec![];
        for str in vector {
          protocol_vector.push(ProtocolType::String(str.to_string()));
        }
        let protocol_string = ProtocolType::Array(protocol_vector).serialize();

        stream.write(protocol_string.as_bytes()).unwrap();

        let mut data = [0; 512];
        match stream.read_exact(&mut data) {
            Ok(_) => {
              Ok((ProtocolType::String(from_utf8(&data).unwrap().to_string())).to_string())
            },
            Err(_) => {
                Err("fail to get response")
            }
        }
    },
    Err(_) => {
      Err("fail to get response")
    }
  }
}