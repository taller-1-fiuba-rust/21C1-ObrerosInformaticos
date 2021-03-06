use std::collections::HashMap;
use std::fs;

const DEFAULT_VERBOSE: u8 = 0;
const DEFAULT_PORT: u16 = 6379;
const DEFAULT_TIMEOUT: u32 = 0;
const DEFAULT_DBFILENAME: &str = "dump.rdb";
const DEFAULT_LOGFILE: &str = "logfile.txt";
const DEFAULT_IP: &str = "127.0.0.1";

//To add a new configuration attribute:
//  1) Add de default value as a constant.
//  2) Add the attribute to the structure definition.
//  3) Add the attribute to the 'new' function.
//  4) Add the check and set to the set_all_params function.
//  5) Add the get_/attribute/ function to return the value.

///The basic configuration for de server. Includes [Verbose, Port, Timeout, DBFilename, LOGfile, IP]
#[allow(dead_code)]
#[derive(Default)]
pub struct Configuration {
    verbose: u8,
    port: u16,
    timeout: u32,
    dbfilename: String,
    logfile: String,
    ip: String,
    config_file: Option<String>,
}

#[allow(dead_code)]
impl Configuration {
    ///Returns the default configuration
    pub fn new() -> Self {
        Configuration {
            verbose: DEFAULT_VERBOSE,
            port: DEFAULT_PORT,
            timeout: DEFAULT_TIMEOUT,
            dbfilename: DEFAULT_DBFILENAME.to_string(),
            logfile: DEFAULT_LOGFILE.to_string(),
            ip: DEFAULT_IP.to_string(),
            config_file: None,
        }
    }

    /// Re-sets the configuration based on a configuration file (.config).
    /// If any problem happens, it returns a string describing the problem.
    pub fn set_config(&mut self, file_path: &str) -> Result<bool, String> {
        let map;
        match self.parse(file_path) {
            Ok(map_) => map = map_,
            Err(err) => return Err(err),
        }

        if let Some(err) = self.set_all_params(map) {
            return Err(err);
        }
        self.config_file = Some(file_path.to_string());

        Ok(true)
    }

    /// Returns a map <Attribute_name, Attribute_value> containing all the attributes
    /// that the file contained.
    /// If any problem happens, it returns a String describing the problem.
    fn parse(&mut self, file_path: &str) -> Result<HashMap<String, String>, String> {
        let file: String = match fs::read_to_string(file_path) {
            Ok(file) => file,
            Err(_) => return Err("Error al intentar abrir el archivo".to_string()),
        };

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
        Ok(map)
    }

    /// Sets all the params and checks the validity of some of them.
    /// If everything is OK, it returns none.
    /// If any problem happens, it returns a string describing the problem.
    fn set_all_params(&mut self, map: HashMap<String, String>) -> Option<String> {
        if let Some(verbose_) = map.get("verbose") {
            if !self.check_number_between(verbose_, 0, 1) {
                return Some("Verbosidad mal configurada.".to_string());
            }
            self.verbose = verbose_.parse().unwrap();
            if self.verbose == 1 {
                println!("Configuraci??n de la verbosidad cargada : 1");
            }
        }

        if let Some(logfile_) = map.get("logfile") {
            self.logfile = logfile_.to_string();
            if self.verbose == 1 {
                println!(
                    "Configuraci??n del nombre del archivo de logs cargada : {}",
                    self.logfile
                );
            }
        }

        if let Some(port_) = map.get("port") {
            if !self.check_number_between(port_, 0, 65536) {
                return Some("Puerto mal configurado.".to_string());
            }
            self.port = port_.parse().unwrap();
            if self.verbose == 1 {
                println!("Configuraci??n del puerto cargada : {}", self.port);
            }
        }

        if let Some(timeout_) = map.get("timeout") {
            match timeout_.parse::<u32>() {
                Ok(number) => self.timeout = number,
                Err(_) => return Some("Timeout mal configurado.".to_string()),
            }
            if self.verbose == 1 {
                println!("Configuraci??n del timeout cargada : {}", self.timeout);
            }
        }

        if let Some(dbfilename_) = map.get("dbfilename") {
            self.dbfilename = dbfilename_.to_string();
            if self.verbose == 1 {
                println!(
                    "Configuraci??n del archivo de almacenamiento cargada : {}",
                    self.dbfilename
                );
            }
        }

        if let Some(ip_) = map.get("ip") {
            self.ip = ip_.to_string();
            if self.verbose == 1 {
                println!("Configuraci??n de la ip cargada : {}", self.ip);
            }
        }
        None
    }

    fn check_number_between(&mut self, number: &str, bottom: u32, top: u32) -> bool {
        let int_number: u32;
        match number.parse::<u32>() {
            Ok(x) => int_number = x,
            Err(_) => return false,
        }

        if int_number <= top && int_number >= bottom {
            return true;
        }
        false
    }

    pub fn get_configfile(&self) -> Option<String> {
        self.config_file.clone()
    }

    pub fn get_verbose(&self) -> u8 {
        self.verbose
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }

    pub fn get_timeout(&self) -> u32 {
        self.timeout
    }

    pub fn get_dbfilename(&self) -> &String {
        &self.dbfilename
    }

    pub fn get_logfile(&self) -> &String {
        &self.logfile
    }

    pub fn get_ip(&self) -> &String {
        &self.ip
    }

    pub fn set_verbose(&mut self, new_verb: u8) -> Result<&'static str, &'static str> {
        if new_verb != 0 && new_verb != 1 {
            return Err("La verbosidad tiene que ser 0 o 1.");
        }
        self.verbose = new_verb;
        Ok("Ok")
    }

    pub fn set_port(&mut self, new_port: u16) {
        self.port = new_port;
    }

    pub fn set_timeout(&mut self, new_timeout: u32) {
        self.timeout = new_timeout;
    }

    pub fn set_dbfilename(&mut self, new_dbfilename: String) {
        self.dbfilename = new_dbfilename;
    }

    pub fn set_logfile(&mut self, new_logfile: String) {
        self.logfile = new_logfile;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_general_configuration() {
        let mut configuration = Configuration::new();

        match configuration.set_config(&"test_files/test_configuration_1.config".to_string()) {
            Err(_) => {
                assert_eq!(true, false)
            }
            Ok(_) => {
                assert_eq!(configuration.get_verbose(), 0);
                assert_eq!(configuration.get_port(), 6060);
                assert_eq!(configuration.get_timeout(), 100);
                assert_eq!(configuration.get_dbfilename(), "andres.config");
                assert_eq!(configuration.get_logfile(), "asda");
            }
        }
    }

    #[test]
    fn test_2_error_while_verbose_different_than_0_or_1() {
        let mut configuration = Configuration::new();

        match configuration.set_config(&"test_files/test_configuration_2.config".to_string()) {
            Err(x) => {
                assert_eq!(x, "Verbosidad mal configurada.");
            }
            Ok(_) => {
                assert_eq!(true, false);
            }
        }
    }

    #[test]
    fn test_3_accepts_less_parameters_and_has_default() {
        let mut configuration = Configuration::new();

        match configuration.set_config(&"test_files/test_configuration_3.config".to_string()) {
            Err(_) => {
                assert_eq!(true, false)
            }
            Ok(_) => {
                assert_eq!(configuration.get_verbose(), 0);
                assert_eq!(configuration.get_port(), 6379);
                assert_eq!(configuration.get_timeout(), 0);
                assert_eq!(configuration.get_dbfilename(), "andres.config");
                assert_eq!(configuration.get_logfile(), "asda");
            }
        }
    }
}
