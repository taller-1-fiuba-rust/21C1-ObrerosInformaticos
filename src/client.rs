use std::net::TcpStream;
use std::sync::{RwLock, RwLockWriteGuard, Mutex};
use crate::protocol::command::Command;
use crate::protocol::request::Request;
use std::io::{BufReader, BufRead, Write, Read};
use std::sync::atomic::{AtomicBool, Ordering, AtomicU64};
use std::hash::{Hash, Hasher};
use std::collections::VecDeque;
use std::time::Duration;

static CLIENT_ID: AtomicU64 = AtomicU64::new(0);

/// A represents Redis client. Wraps a TCP socket and some state.
pub struct Client {
    write_socket: Mutex<TcpStream>,
    read_socket: Mutex<TcpStream>,
    in_pubsub: AtomicBool,
    closed: AtomicBool,
    id: u64,
}

impl Client {
    /// Returns a new client from a socket
    pub fn new(socket: TcpStream) -> Self {
        Client {
            read_socket: Mutex::new(socket.try_clone().unwrap()),
            write_socket: Mutex::new(socket),
            in_pubsub: AtomicBool::new(false),
            closed: AtomicBool::new(false),
            id: CLIENT_ID.fetch_add(1, Ordering::SeqCst),
        }
    }

    /// Gets if its in pubsub mode
    pub fn in_pubsub_mode(&self) -> bool {
        self.in_pubsub.load(Ordering::SeqCst)
    }

    /// Sets the client in pubsub mode
    pub fn set_pubsub_mode(&self, new: bool) {
        self.in_pubsub.store(new, Ordering::SeqCst);
    }

    /// Returns a bool representing if the client closed the connection
    pub fn is_closed(&self) -> bool {
        self.closed.load(Ordering::SeqCst)
    }

    /// Send a string message to this client
    pub fn send(&self, msg: &str) -> Result<(), &'static str> {
        let mut lock = self.write_socket.lock().ok().ok_or_else(||"Failed to lock socket")?;
        lock.write_all(msg.as_bytes()).ok().ok_or("Error while writing to client")?;
        Ok(())
    }

    /// Parses a command from a socket connection
    pub fn parse_commands(&self) -> Result<Vec<Command>, String> {
        let mut locked_socket = self.read_socket.lock().ok().ok_or_else(||"Failed to lock socket".to_string())?;
        let mut request = Request::new();
        let mut result: Result<bool, String> = Err("Empty message".to_string());
        let mut commands = Vec::new();
        let copy = locked_socket.try_clone().unwrap();
        copy.set_read_timeout(Some(Duration::from_millis(10))).unwrap();
        let mut reader = BufReader::new(copy);
        let mut line = String::new();
        let mut offset = 0;

        loop {
            let read_result = reader.read_line(&mut line);
            match read_result {
                Ok(s) => {
                    if s == 0 {
                        self.closed.store(true, Ordering::SeqCst);
                        return Err("Client closed the connection".to_string());
                    } else {
                        let l = &line[offset..offset+s];
                        offset += s;
                        println!("{}", &l.replace("\r\n", ""));
                        result = request.feed(&l);
                        if let Ok(val) = result {
                            if val {
                                commands.push(request.build());
                                request = Request::new();
                            } else {}
                        } else if let Err(e) = result {
                            return Err(e);
                        }
                    }
                },
                Err(_) => {
                    if commands.len() > 0 {
                        break;
                    }
                }
            }
        }

        Ok(commands)
    }
}

impl PartialEq for Client {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl Eq for Client {

}

impl Hash for Client {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}