use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

pub fn read_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf.lines()
                                .map(|l| l.expect("Could not parse line"))
                                .collect();

    lines
}