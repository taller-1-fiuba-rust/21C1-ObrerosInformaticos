use std::collections::HashMap;
use crate::storage::data_storage::Value;
use crate::storage::file_reader;

pub fn parse_data(file: &str, data: &mut HashMap<String, Value>){
	let lines = file_reader::read_lines(file);

	for line in lines{
		let vec: Vec<&str> = line
    					     .split(|c| c == ';')
    					     .collect();
    	if vec[0].contains("[") {
    		//data.insert(vec[0].to_string(), Value::Vec(vec[1]));
    	}else if vec[0].contains("{") {
    		//data.insert(vec[0].to_string(), Value::HashSet(vec[1]));
    	}else {
    		data.insert(vec[0].to_string(), Value::String(vec[1].to_string()));
    	};
	}
}

pub fn store_data(file: &str, data: &HashMap<String, Value>){
	print!("Saving data..");
}