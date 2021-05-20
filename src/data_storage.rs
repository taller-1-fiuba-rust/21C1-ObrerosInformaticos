use std::collections::HashMap;
use std::collections::HashSet;

pub enum Value {
    String(String),
    Vec(Vec<String>),
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

    pub fn add_key_value(&mut self, key: &str, value: Value){
    	
    	let copy_key = key.to_string();

    	match value {
    		Value::String(s) => self.data.insert(copy_key, Value::String(s)),
    		Value::Vec(i)    => self.data.insert(copy_key, Value::Vec(i)),
    		Value::HashSet(j) => self.data.insert(copy_key, Value::HashSet(j)),
		};
    }

    //TODO: Cuando se implementen los comandos hay que hacer funciones que eliminen o solo el primer valor
    // del vector o el ultimo dada una clave. Aca solo se borra la clave con todo lo que contiene.
    pub fn delete_key(&mut self, key: &str){
    	self.data.remove(key);
    }

    pub fn get_value(&self, key: &str) -> Option<&Value> {
    	self.data.get(key)
    }

}

#[cfg(test)]

mod tests {
	use super::*;

    #[test]
    fn test_add_string_data(){

    	let mut data_storage = DataStorage::new();
    	let key = String::from("Daniela");
    	let value = String::from("hola");

    	data_storage.add_key_value(&key, Value::String(value));

    	let b = if let Value::String(a) = data_storage.get_value(&key).unwrap() {
    		a
    	} else {
    		panic!("Not string value")
    	};

    	assert_eq!("hola", *b);
    } 

    #[test]
    fn test_add_vector_data(){

    	let mut data_storage = DataStorage::new();
    	let key = String::from("Daniela");
    	let value = vec!["a".to_string(), "b".to_string()];

    	data_storage.add_key_value(&key, Value::Vec(value));

    	let b = if let Value::Vec(a) = data_storage.get_value(&key).unwrap() {
    		a
    	} else {
    		panic!("Not string value")
    	};

    	assert_eq!(vec!["a", "b"], *b);
    }

    #[test]
    fn test_add_set_data(){

    	let mut data_storage = DataStorage::new();
    	let key = String::from("Daniela");
    	let value: HashSet<String> = vec!["a".to_string(), "b".to_string()].into_iter().collect();

    	data_storage.add_key_value(&key, Value::HashSet(value));

    	let b = if let Value::HashSet(a) = data_storage.get_value(&key).unwrap() {
    		a
    	} else {
    		panic!("Not string value")
    	};

    	let a: HashSet<String> = vec!["a".to_string(), "b".to_string()].into_iter().collect();

    	assert_eq!(a, *b);
    }

    #[test]
    #[should_panic]
    fn test_delete_data(){

    	let mut data_storage = DataStorage::new();
    	let key = String::from("Daniela");
    	let value = String::from("hola");

    	data_storage.add_key_value(&key, Value::String(value));

    	data_storage.delete_key(&key);

    	if let Value::String(a) = data_storage.get_value(&key).unwrap() {
    		a
    	} else {
    		panic!("Value not found in storage")
    	};
    } 

}

