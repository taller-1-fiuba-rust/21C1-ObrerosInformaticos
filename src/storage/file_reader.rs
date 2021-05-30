use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;

///Lee todas las lineas de un archivo y lo devuelve en 
///formato de vector. Cada elemento del vector es una linea
///del archivo.
///PRE: El archivo debe existir.
///POST: Se devuelve un vector con todo el contenido del 
///archivo en el.
pub fn read_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    lines
}

///Dado un nombre de archivo y un string almacena el 
///string en el final del archivo. No borra la informacion
///ya existente del archivo, la misma se agrega al final.
pub fn data_to_file(file: &str, data: String) {
    let mut file = OpenOptions::new()
        .append(true)
        .open(file)
        .expect("Unable to open");

    file.write_all(data.as_bytes())
        .expect("Unable to write file");
    file.write_all("\n".as_bytes())
        .expect("Unable to write file");
}
