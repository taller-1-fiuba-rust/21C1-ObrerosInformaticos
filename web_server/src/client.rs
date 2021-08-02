use std::net::{TcpStream};
use std::io::{Read, Write};
use redis_protocol::types::ProtocolType;
use std::str::from_utf8;


pub fn send_request(connection_port: String, request: &String) -> Result<String, &'static str> {

  match TcpStream::connect(connection_port) {
    Ok(mut stream) => {
        //TOMAR EL STRING DEL REQUEST COMO UN PROTOCOLTYPE Y SERIALIZARLO PARA ENVIARLO POR TCP.
        stream.write(request.as_bytes()).unwrap();

        let mut data = [0; 512];
        match stream.read_exact(&mut data) {
            Ok(_) => {
              Ok((ProtocolType::String(from_utf8(&data).unwrap().to_string())).to_string())
            },
            Err(e) => {
                Err(&ProtocolType::String("fail to get response".to_string()).to_string())
            }
        }
    },
    Err(e) => {
      Err(&ProtocolType::String("fail to get response".to_string()).to_string())
    }
  }
}