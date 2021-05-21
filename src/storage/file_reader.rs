use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;

pub fn read_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf.lines()
                                .map(|l| l.expect("Could not parse line"))
                                .collect();

    lines
}

pub fn data_to_file(file: &str, data: String){

	let mut file = OpenOptions::new()
    				.append(true)
    				.open(file)
    				.expect("Unable to open");

    file.write_all(data.as_bytes()).expect("Unable to write file");
    file.write_all("\n".as_bytes()).expect("Unable to write file");
}

