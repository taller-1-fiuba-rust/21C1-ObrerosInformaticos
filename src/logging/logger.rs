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
    Message(String),
    File(File),
    Terminate,
}

impl Logger {
    pub fn new(file_dir: &str) -> Result<Self, &'static str> {
        let mut file = create_logfile(file_dir)?;
        let (sender, receiver): (Sender<Message>, Receiver<Message>) = mpsc::channel();
        thread::spawn(move || loop {
            let message = receiver.recv();
            if let Ok(msg) = message {
                match msg {
                    Message::Message(string) => write(&string, &file),
                    Message::File(new_file) => {
                        file = new_file;
                    }
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
                if sender.send(Message::Message(msg.to_string())).is_err() {
                    return Err("No se pudo loggear el mensaje.");
                };
            }
            Err(_) => return Err("No se pudo loggear el mensaje."),
        }
        Ok(())
    }

    pub fn change_logfile_name(&self, new_name: String) -> Result<(), &'static str> {
        let file = create_logfile(&new_name)?;
        match self.sender.lock() {
            Ok(sender) => {
                if sender.send(Message::File(file)).is_err() {
                    return Err("No se pudo loggear el mensaje");
                }
            }
            Err(_) => return Err("No se pudo cambiar el nombre de archivo"),
        }
        Ok(())
    }
}

impl Drop for Logger {
    #[allow(dead_code)]
    fn drop(&mut self) {
        let sender = self.sender.lock().unwrap();
        sender.send(Message::Terminate).unwrap();
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
