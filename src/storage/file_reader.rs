use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;

/// Read all the lines of a file and return it in
/// vector format. Each element of the vector is a line
/// of the file.
/// PRE: The file must exist.
/// POST: A vector is returned with all the content of the
/// file in the.
pub fn read_lines(filename: &str) -> Result<Vec<String>, &'static str> {
    let file = File::open(filename);
    match file {
        Ok(file_name) => {
            let buf = BufReader::new(file_name);
            let lines: Vec<String> = buf
                .lines()
                .map(|l| l.expect("Could not parse line"))
                .collect();

            Ok(lines)
        }
        Err(_i) => Err("Not existing file"),
    }
}

/// Given a filename and a string stores the
/// string at the end of the file. It does not erase the information
/// already existing file, it is added to the end.
pub fn data_to_file(file: &str, data: String) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)   
        .open(file)
        .expect("Unable to open");

    file.write_all(data.as_bytes())
        .expect("Unable to write file");
    file.write_all("\n".as_bytes())
        .expect("Unable to write file");
}
