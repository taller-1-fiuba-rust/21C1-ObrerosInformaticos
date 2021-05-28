use std::fs::remove_file;
use std::fs::File;
use std::io::prelude::*;

pub struct Logger {
    file_handle: Option<File>,
    file_dir: Option<String>,
}

impl Logger {
    pub fn new() -> Logger {
        Logger {
            file_handle: None,
            file_dir: None,
        }
    }

    pub fn set_logfile(&mut self, file_dir: &str) -> Option<String> {
        if let Some(dir) = self.file_dir.clone() {
            if let Err(_) = remove_file(dir) {
                return Some("No se pudo borrar el archivo de logs por default.".to_string());
            }
        }

        if let Ok(file) = File::create(file_dir) {
            self.file_handle = Some(file);
            self.file_dir = Some(file_dir.to_string());
            self.log("Log file creado.");
            return None;
        }
        return Some("No se pudo crear el archivo de logs.".to_string());
    }

    pub fn log(&mut self, msg: &str) -> bool {
        if let None = self.file_handle {
            return false;
        }
        if let Err(_) = self
            .file_handle
            .as_ref()
            .unwrap()
            .write(format!("{}{}", msg, '\n').as_bytes())
        {
            return false;
        }
        true
    }
}
