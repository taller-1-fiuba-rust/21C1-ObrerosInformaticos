use redis_protocol::parser::ParserFactory;
use redis_protocol::types::ProtocolType;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

pub fn send_request(connection_port: String, request: &str) -> Result<Vec<u8>, &'static str> {
    match TcpStream::connect(connection_port) {
        Ok(mut stream) => {

            write_query(&mut stream, request)?;

            let response_contents = read_response(&mut stream)?;
            let response_str = String::from_utf8(response_contents).ok().ok_or("Invalid response from redis")?;

            parse_resp_into_bytes(&response_str)
        }
        Err(_) => Err("fail to get response"),
    }
}

fn write_query(stream: &mut TcpStream, request: &str) -> Result<(), &'static str> {
    let protocol_string = ProtocolType::Array(request.split(' ').map(|x| x.to_owned()).map(ProtocolType::String).collect()).serialize();
    stream.write_all(protocol_string.as_bytes())
        .ok()
        .ok_or("Failed to read from socket")
}

fn read_response(stream: &mut TcpStream) -> Result<Vec<u8>, &'static str> {
    stream
        .set_read_timeout(Some(Duration::from_millis(1)))
        .ok()
        .ok_or("Failed to read from socket")?;

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
    Ok(response_contents)
}

fn parse_resp_into_bytes(response_str: &str) -> Result<Vec<u8>, &'static str> {
    let mut chars = response_str.chars();
    return if let Some(mut parser) = ParserFactory::create(chars.next().unwrap()) {
        for line in response_str.split("\r\n") {
            println!("{}", line);
            if parser.feed(&format!("{}\r\n", line)).unwrap() {
                break;
            }
        }
        Ok(parser.build().to_string().as_bytes().to_owned())
    } else {
        Err("Invalid RESP string")
    };
}
