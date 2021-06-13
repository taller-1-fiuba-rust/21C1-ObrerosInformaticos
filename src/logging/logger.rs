use std::fs::File;
use std::io::prelude::*;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Mutex;
use std::thread;

pub struct Logger {
    sender: Mutex<Sender<Message>>,
}

enum Message {
    String(String),
    Terminate
}

impl Logger {
    pub fn new(file_dir: &str) -> Result<Self, &'static str> {
        let file = create_logfile(file_dir)?;
        let (sender, receiver): (Sender<Message>, Receiver<Message>) = mpsc::channel();
        thread::spawn(move || loop {
            let message = receiver.recv();
            if let Ok(msg) = message {
                match msg {
                    Message::String(string) => write(&string, &file),
                    Message::Terminate => break,
                }
            }
        });
        let sender_mutex = Mutex::new(sender);
        Ok(Logger {
            sender: sender_mutex,
        })
    }

    pub fn log(&self, msg: &str) -> Result<(), &'static str> {
        match self.sender.lock() {
            Ok(sender) => {
                if sender.send(Message::String(msg.to_string())).is_err() {
                    return Err("No se pudo loggear el mensaje.");
                };
            }
            Err(_) => return Err("No se pudo loggear el mensaje."),
        }
        Ok(())
    }

    pub fn drop(&self) -> Result<(), &'static str> {
        match self.sender.lock() {
            Ok(sender) => {
                if sender.send(Message::Terminate).is_err() {
                    return Err("No se pudo loggear el mensaje.");
                };
            }
            Err(_) => return Err("No se pudo loggear el mensaje."),
        }
        Ok(())
    }
}

pub fn create_logfile(file_dir: &str) -> Result<File, &'static str> {
    if let Ok(file) = File::create(file_dir) {
        return Ok(file);
    }
    Err("No se pudo crear el archivo de logs.")
}

pub fn write(msg: &str, mut file: &File) {
    if file.write(format!("{}{}", msg, '\n').as_bytes()).is_err() {
        println!("No se pudo escribir en el archivo");
    }
}
