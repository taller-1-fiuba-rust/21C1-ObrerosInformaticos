use std::collections::HashMap;
use std::collections::HashSet;

pub enum Value {
    String(String),
    Vec(String),
    HashSet(HashSet<String>),
}

pub struct DataStorage {
	data: HashMap<String, Value>,
}

impl DataStorage {

	pub fn new() -> Self {
		let data: HashMap<String, Value> = HashMap::new(); 
        DataStorage { data }
    }

    pub fn add_data(&mut self, key: String, value: Value) -> Result<() , &'static str>{
    	
    	match value {
    		Value::String(s) => self.data.insert(key, Value::String(s)),
    		Value::Vec(i)    => self.data.insert(key, Value::Vec(i)),
    		Value::HashSet(j) => self.data.insert(key, Value::HashSet(j)),
		};

		Ok(())
    }
}

#[cfg(test)]

mod tests {
	use super::*;

	#[test]
    fn test_create_new_data_storage(){
    	let data_storage = DataStorage::new();
    } 

}

