use std::collections::HashMap;
use std::collections::HashSet;
use crate::storage::data_storage::Value;
use crate::storage::file_reader;

static LIST: &str = "|LISTA|";
static SET: &str = "|SET|";

pub fn parse_data(file: &str, data: &mut HashMap<String, Value>){
	let lines = file_reader::read_lines(file);

	for line in lines{

		let vec: Vec<&str> = line.split(|c| c == ';' || c == ',')
    					         .collect();

    	if vec[1].contains(LIST){

    		let (key, vector) = get_vector_data(vec);
    		data.insert(key, Value::Vec(vector));
    	}else if vec[1].contains(SET){

    		let (key, set) = get_set_data(vec);
    		data.insert(key, Value::HashSet(set));
    	}else{

    		data.insert(vec[0].to_string(), Value::String(vec[1].to_string()));
    	};
	}
}

pub fn store_data(file: &str, data: &HashMap<String, Value>){
	
	for (key, value) in &*data {

		match value {
    		Value::String(s) => save_string_data(file, key, s),
    		Value::Vec(i)    => save_vector_data(file, key, i),
    		Value::HashSet(j) => save_set_data(file, key, j),
		};
	} 
}

fn save_string_data(file: &str, key: &str, value: &str){
	let save_data: String = key.to_owned() + &";".to_string() + value;
	file_reader::data_to_file(file, save_data);
}

fn save_vector_data(file: &str, key: &str, value: &[String]){
	let mut save_data: String = key.to_owned() + &";".to_string() + LIST;

	for element in value{
		save_data = save_data + &";".to_string() + element;
	}

	file_reader::data_to_file(file, save_data);
}

fn save_set_data(file: &str, key: &str, value: &HashSet<String>){
	let mut save_data: String = key.to_owned() + &";".to_string() + SET;

	for element in value{
		save_data = save_data + &";".to_string() + element;
	}

	file_reader::data_to_file(file, save_data);

}

fn get_vector_data(mut vec: Vec<&str>) -> (String, Vec<String>) {
	let mut data: Vec<String> = vec![];
	let key = vec[0].to_string();
	vec.remove(0);
	vec.remove(0);

	for element in vec {
		data.push(element.to_string());
	}

	(key, data)
}

fn get_set_data(mut vec: Vec<&str>) -> (String, HashSet<String>) {
	let mut data: HashSet<String> = HashSet::new();
	let key = vec[0].to_string();
	vec.remove(0);
	vec.remove(0);

	for element in vec {
		data.insert(element.to_string());
	}

	(key, data)
}