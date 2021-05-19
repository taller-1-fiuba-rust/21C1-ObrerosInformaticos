use std::fs::File;

pub struct Configuration {
    verbose: i8,
    port: i16,
    timeout: i64,
    dbfilename: String,
    logfile: String,

}

impl Configuration {
    pub fn new(file_path: &String) -> Self{

        Configuration{
            verbose: 1,
            port: 1,
            timeout: 1,
            dbfilename: "asd".to_string(),
            logfile: "asd".to_string()
        }
    }
    
    fn parse(&mut self, file_path: &String) -> Vec<_> {

    }    

    fn open_file(&mut self, file_path: &String) -> {
        let file = File::open(file_path)?;
    }
}
