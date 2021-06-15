use std::net::TcpStream;
use std::sync::{RwLock, RwLockWriteGuard, Mutex};
use crate::protocol::command::Command;
use crate::protocol::request::Request;
use std::io::{BufReader, BufRead, Write};
use std::sync::atomic::{AtomicBool, Ordering, AtomicU64};
use std::hash::{Hash, Hasher};
use std::collections::VecDeque;

static CLIENT_ID: AtomicU64 = AtomicU64::new(0);

/// A represents Redis client. Wraps a TCP socket and some state.
pub struct Client {
    socket: RwLock<TcpStream>,
    in_pubsub: AtomicBool,
    msg_queue: Mutex<VecDeque<String>>,
    id: u64,
}

impl Client {
    /// Returns a new client from a socket
    pub fn new(socket: TcpStream) -> Self {
        //socket.set_nonblocking(true).unwrap();
        Client {
            socket: RwLock::new(socket),
            in_pubsub: AtomicBool::new(false),
            msg_queue: Mutex::new(VecDeque::new()),
            id: CLIENT_ID.fetch_add(1, Ordering::SeqCst)
        }
    }

    /// Gets if the client is in pubsub mode
    pub fn is_pubsub_mode(&self) -> bool {
        false//self.in_pubsub.into_inner().clone()
    }

    /// Sets the client in pubsub mode
    pub fn set_pubsub_mode(&self, new: bool) {
        if new {
            self.in_pubsub.fetch_or(new, Ordering::SeqCst);
        } else {
            self.in_pubsub.fetch_and(new, Ordering::SeqCst);
        }
    }

    /// Returns a bool representing if the client closed the connection
    pub fn is_closed(&self) -> bool {
        match self.peek() {
            Ok(i) => i == 0,
            _ => false
        }
    }

    /// Queue a string message to this client
    pub fn send(&self, msg: &str) -> Result<(), &'static str> {
        if let Ok(mut l) = self.socket.try_write() {
            self.do_send(&mut l, msg)
        } else {
            let mut queue = self.msg_queue.lock().ok().ok_or("Failed to lock socket")?;
            queue.push_back(msg.to_string());
            Ok(())
        }
    }

    /// Actually send the message
    fn do_send(&self, socket: &mut RwLockWriteGuard<TcpStream>, msg: &str) -> Result<(), &'static str> {
        socket.write_all(msg.as_bytes()).ok().ok_or("Error while writing to client")?;
        Ok(())
    }

    /// Flush queued messages to the socket
    fn flush_messages(&self) -> Result<(), &'static str> {
        let mut queue = self.msg_queue.lock().ok().ok_or("Failed to lock socket")?;
        let mut locked_socket = self.socket.write().ok().ok_or("Failed to lock socket")?;

        while let Some(s) = queue.pop_front() {
            self.do_send(&mut locked_socket, &s)?;
        }
        Ok(())
    }

    /// Peek the next byte
    fn peek(&self) -> Result<usize, &'static str> {
        let locked_socket = match self.socket.read() {
            Ok(l) => l,
            Err(_) => return Err("Failed to lock")
        };
        locked_socket.set_nonblocking(true).unwrap();
        let mut buf = [0;1];
        let result = match locked_socket.peek(&mut buf) {
            Ok(i) => Ok(i),
            Err(_) => Err("Failed to peek"),
        };
        locked_socket.set_nonblocking(false).unwrap();
        result
    }

    /// Is the client waiting for a message to be processed by the server?
    fn has_command(&self) -> bool {
        match self.peek() {
            Ok(i) => i == 1,
            _ => false
        }
    }

    /// Parses a command from a socket connection
    pub fn parse_command(&self) -> Result<Command, String> {
        while !self.has_command() {
            //println!("waiting..");
            let queue = self.msg_queue.lock().ok().ok_or_else(|| "Failed to lock socket".to_string())?;
            if !queue.is_empty() {
                drop(queue);
                self.flush_messages()?;
            }
        }
        self.do_parse_command()
    }

    /// Parses a command from a socket connection
    fn do_parse_command(&self) -> Result<Command, String> {
        let locked_socket = self.socket.read().ok().ok_or_else(||"Failed to lock socket".to_string())?;
        let mut request = Request::new();
        let reader = BufReader::new(locked_socket.try_clone().unwrap());
        let mut result: Result<bool, String> = Err("Empty message".to_string());

        for line in reader.lines() {
            let l = line.unwrap();
            let formatted = format!("{}\r\n", &l);
            result = request.feed(&formatted);
            if let Ok(val) = result {
                if val {
                    break;
                } else {
                }
            } else if result.is_err() {
                break;
            }
        }
        if let Err(e) = result {
            return Err(e);
        }

        Ok(request.build())
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