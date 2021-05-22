use std::collections::HashMap;
use std::fs;

pub enum ConfigLoadError {
    MissingConfValues,
    FilePathError,
    ConfigValuesError
}

pub struct Configuration <'a> {
    verbose: i8,
    port: i16,
    timeout: i64,
    dbfilename: &'a String,
    logfile: &'a String,
}

impl <'a> Configuration <'_> {
    pub fn new() -> Self {
        //Devuelvo configuración por defecto
        Configuration {
            verbose: 0,
            port: 8080,
            timeout: 0,
            dbfilename: &"".to_string(),
            logfile: &"".to_string(),
        }
    }

    pub fn set_config(&mut self, file_path: &'a String) -> Result<bool, ConfigLoadError> {
        if !self.check_file_path(file_path) {
            return Err(ConfigLoadError::FilePathError);
        }

        let (verbose, port, timeout, dbfilename, logfile);
        match self.parse(file_path) {
            Ok((verbose_, port_, timeout_, dbfilename_, logfile_)) => {
                verbose = verbose_;
                port = port_;
                timeout = timeout_;
                dbfilename = dbfilename_;
                logfile = logfile_;
            }
            Err(err) => return Err(err),
        }

        self.verbose = verbose;
        self.port = port;
        self.timeout = timeout;
        self.dbfilename = dbfilename;
        self.logfile = logfile;
        Ok(true)
    }

    pub fn get_verbose(&mut self) -> i8 {
        self.verbose
    }

    pub fn get_port(&mut self) -> i16 {
        self.port
    }

    pub fn get_timeout(&mut self) -> i64 {
        self.timeout
    }

    pub fn get_dbfilename(&mut self) -> &String {
        &self.dbfilename
    }

    pub fn get_logfile(&mut self) -> &String {
        &self.logfile
    }

    fn check_file_path(&mut self, file_path: &String) -> bool {
        if *file_path == "asd" {
            return false;
        }
        true
    }
    
    fn parse(&mut self, file_path: &String) -> Result<(i8, i16, i64, &String, &String), ConfigLoadError> {
        let file: String =
            fs::read_to_string(file_path).expect("Algo salió mal al abrir el archivo");
    
        let mut map: HashMap<String, String> = HashMap::new();
    
        let lines = file.lines();
    
        for line in lines {
            let name_and_value: Vec<&str> = line.split('=').collect();
            let config_name: String = name_and_value[0]
                .to_lowercase()
                .replace(' ', "")
                .to_string();
            let value: String = name_and_value[1].replace(' ', "").to_string();
            map.insert(config_name, value);
        }
    
        let port: i16;
        let verbose: i8;
        let timeout: i64;
        let dbfilename : &String;
        let logfile : &String;
    
        match map.get("verbose"){
            Some(verbose_) => {
                if !self.check_number_between(verbose_, 0, 1){
                    return Err(ConfigLoadError::ConfigValuesError)
                }
                verbose = verbose_.parse().unwrap();
            },
            None => return Err(ConfigLoadError::MissingConfValues)
        }
    
        match map.get("port"){
            Some(port_) => {
                if !self.check_number_between(port_, 0, 65536){
                    return Err(ConfigLoadError::ConfigValuesError)
                }
                port = port_.parse().unwrap();
            },
            None => return Err(ConfigLoadError::MissingConfValues)
        }
    
        match map.get("timeout"){
            Some(timeout_) => {
                match self.check_parseable_to_i64(timeout_) {
                    Some(number) => timeout = number,
                    None => return Err(ConfigLoadError::ConfigValuesError)
                }
            },
            None => return Err(ConfigLoadError::MissingConfValues)
        }
    
        match map.get("dbfilename"){
            Some(dbfilename_) => {
                dbfilename = dbfilename_;
            },
            None => return Err(ConfigLoadError::MissingConfValues)
        }
    
        match map.get("logfile"){
            Some(logfile_) => {
                logfile = logfile_;
            },
            None => return Err(ConfigLoadError::MissingConfValues)
        }
        
        Ok((verbose, port, timeout, dbfilename, logfile))
    }
    
    fn check_number_between(&mut self, number: &String, bottom: i64, top: i64) -> bool{
        // let result_number : Result<i64, _> = number.parse::<i64>();
        let int_number : i64;
        match self.check_parseable_to_i64(number) {
            Some(x) => int_number = x,
            None => return false
        }
    
        if int_number <= top && int_number >= bottom {
            return true;
        }
        false
    }
    
    fn check_parseable_to_i64(&mut self, number: &String ) -> Option<i64>{
        let result_number : Result<i64, _> = number.parse::<i64>();
        match result_number {
            Ok(x) => return Some(x),
            Err(_) => return None
        }
    }
}