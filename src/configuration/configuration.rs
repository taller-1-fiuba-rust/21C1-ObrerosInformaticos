use std::collections::HashMap;
use std::fs;

const DEFAULT_VERBOSE: i8 = 0;
const DEFAULT_PORT: i16 = 8080;
const DEFAULT_TIMEOUT: i64 = 0;
const DEFAULT_DBFILENAME: &str = "dump.rdb";
const DEFAULT_LOGFILE: &str = "logfile";

#[allow(dead_code)]
pub struct Configuration {
    verbose: i8,
    port: i16,
    timeout: i64,
    dbfilename: String,
    logfile: String,
}

#[allow(dead_code)]
impl Configuration {
    pub fn new() -> Self {
        //Devuelvo configuración por defecto
        Configuration {
            verbose: DEFAULT_VERBOSE,
            port: DEFAULT_PORT,
            timeout: DEFAULT_TIMEOUT,
            dbfilename: DEFAULT_DBFILENAME.to_string(),
            logfile: DEFAULT_LOGFILE.to_string(),
        }
    }

    pub fn set_config(&mut self, file_path: &str) -> Result<bool, String> {
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

    fn parse(&mut self, file_path: &str) -> Result<(i8, i16, i64, String, String), String> {
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

        let mut port: i16 = DEFAULT_PORT;
        let mut verbose: i8 = DEFAULT_VERBOSE;
        let mut timeout: i64 = DEFAULT_TIMEOUT;
        let mut dbfilename: String = DEFAULT_DBFILENAME.to_string();
        let mut logfile: String = DEFAULT_LOGFILE.to_string();

        if let Some(verbose_) = map.get("verbose") {
            if !self.check_number_between(verbose_, 0, 1) {
                return Err("Verbosidad mal configurada.".to_string());
            }
            verbose = verbose_.parse().unwrap();
        }

        if let Some(port_) = map.get("port") {
            if !self.check_number_between(port_, 0, 65536) {
                return Err("Puerto mal configurado.".to_string());
            }
            port = port_.parse().unwrap();
        }

        if let Some(timeout_) = map.get("timeout") {
            match timeout_.parse::<i64>() {
                Ok(number) => timeout = number,
                Err(_) => return Err("Timeout mal configurado.".to_string()),
            }
        }

        if let Some(dbfilename_) = map.get("dbfilename") {
            dbfilename = dbfilename_.to_string();
        }

        if let Some(logfile_) = map.get("logfile") {
            logfile = logfile_.to_string();
        }

        Ok((verbose, port, timeout, dbfilename, logfile))
    }

    fn check_number_between(&mut self, number: &String, bottom: i64, top: i64) -> bool {
        let int_number: i64;
        match number.parse::<i64>() {
            Ok(x) => int_number = x,
            Err(_) => return false,
        }

        if int_number <= top && int_number >= bottom {
            return true;
        }
        false
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
                assert_eq!(configuration.get_port(), 8080);
                assert_eq!(configuration.get_timeout(), 0);
                assert_eq!(configuration.get_dbfilename(), "andres.config");
                assert_eq!(configuration.get_logfile(), "asda");
            }
        }
    }
}
