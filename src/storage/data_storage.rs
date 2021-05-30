use crate::storage::parser;
use crate::storage::SafeDataStorage;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::RwLockReadGuard;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

///Enum Value. Contiene todos los tipos de datos soportados
///para el DataStorage.
#[allow(dead_code)]
pub enum Value {
    String(String),
    Vec(Vec<String>),
    HashSet(HashSet<String>),
}

///Struct DataStorage. Se encuentra compuesto por un 
///HashMap el cual almacena la informacion del programa.
///Estructura protegida por un RwLock.
#[allow(dead_code)]
pub struct DataStorage {
    data: SafeDataStorage,
}

///Implementacion de la estructura DataStorage.
#[allow(dead_code)]
impl DataStorage {

    ///Crea la estructura DataStorage. 
    pub fn new() -> Self {
        DataStorage {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    ///Dado un nombre de archivo carga en la base de datos 
    ///la informacion contenida en el mismo.
    ///PRE: El archivo debe tener la estructura soportada 
    ///para la carga de datos y la estructura debe encontrarse
    ///previamente creada.
    ///POST: DataStorage se encuentra cargado con los datos 
    ///que contenia el archivo.
    pub fn load_data(&mut self, file: &str) {
        let mut lock = self.data.write().unwrap();
        parser::parse_data(file, &mut lock);
    }

    ///Dado un nombre de archivo guarda los datos de la 
    ///base de datos en el mismo.
    ///PRE: La estructura DataStorage debe estar creada.
    ///POST: El archivo contiene la informacion que habia 
    ///en la estructura.
    pub fn save_data(&mut self, file: &str) {
        let lock = self.data.read().unwrap();
        parser::store_data(file, &lock);
    }

    ///Dada una clave y un valor los alamacena en la base de datos.
    ///PRE: La estructura DataStorage debe estar creada.
    ///POST: La clave es almacenada en la estructura con su valor
    ///correspondiente y con tiempo de vencimiento 0 dado que las 
    ///claves por default nunca expiran.
    pub fn add_key_value(&mut self, key: &str, value: Value) {
        let mut lock = self.data.write().unwrap();
        let copy_key = key.to_string();

        match value {
            Value::String(s) => lock.insert(copy_key, (None, Value::String(s))),
            Value::Vec(i) => lock.insert(copy_key, (None, Value::Vec(i))),
            Value::HashSet(j) => lock.insert(copy_key, (None, Value::HashSet(j))),
        };
    }

    ///Elimina la clave con su correspondiente valor de la estructura.
    pub fn delete_key(&self, key: &str) -> Result<(), &'static str> {
        let mut lock = self.data.write().unwrap();
        match lock.remove(key) {
            Some(_a) => Ok(()),
            None => Err("Not key in HashMap"),
        }
    }

    pub fn read(&self) -> RwLockReadGuard<'_, HashMap<String, (Option<Duration>, Value)>> {
        self.data.read().unwrap()
    }

    pub fn set_expiration_to_key(
        &self,
        actual_time: SystemTime,
        duration: Duration,
        key: &str,
    ) -> Result<u64, &'static str> {
        let mut lock = self.data.write().unwrap();
        let copy_key = key.to_string();

        let expiration_time = actual_time.checked_add(duration);

        if expiration_time == None {
            Err("Expiration time cant be calculated")
        } else if lock.contains_key(&copy_key) {
            let key_duration = expiration_time.unwrap().duration_since(UNIX_EPOCH);
            lock.get_mut(&copy_key).unwrap().0 = Some(key_duration.unwrap());
            Ok(1)
        } else {
            Err("Key not found in DataStorage")
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use std::env;
    use std::fs::File;
    use std::io::Write;

    #[test]
    #[should_panic]
    fn test_delete_data() {
        let mut data_storage = DataStorage::new();
        let key = String::from("Daniela");
        let value = String::from("hola");

        data_storage.add_key_value(&key, Value::String(value));

        data_storage.delete_key(&key);
        let read = data_storage.read();

        if let Value::String(a) = &(*read.get(&key).unwrap()).1 {
            a
        } else {
            panic!("Value not found in storage")
        };
    }

    #[test]
    fn test_set_exiration_to_key() {
        let mut data_storage = DataStorage::new();
        let key = String::from("Daniela");
        let value = String::from("hola");
        let actual_time = SystemTime::now();
        let duration = Duration::from_secs(5);
        let expiration_time = actual_time.checked_add(duration);
        let key_duration = expiration_time.unwrap().duration_since(UNIX_EPOCH);

        data_storage.add_key_value(&key, Value::String(value));

        let _result = match data_storage.set_expiration_to_key(actual_time, duration, &key) {
            Ok(s) => s,
            Err(_s) => panic!("Key expiration cant be set"),
        };

        let read = data_storage.read();
        let key_expiration: &Option<Duration> = &(*read.get(&key).unwrap()).0;

        assert_eq!(key_duration.unwrap(), key_expiration.unwrap());
    }

    #[test]
    fn test_load_string_data() {
        let dir = env::temp_dir();
        let path_str = dir.to_str().unwrap().to_string() + &"/string_data.txt".to_string();
        let path = dir.to_str().unwrap().to_string() + &"/string_data.txt".to_string();

        let mut file = File::create(path).expect("Not file created");

        writeln!(file, "Daniela;0;hola").expect("Not file write");
        let mut data_storage = DataStorage::new();
        data_storage.load_data(&path_str);

        let key = String::from("Daniela");
        let value = String::from("hola");

        let read = data_storage.read();

        let b = if let Value::String(a) = &(*read.get(&key).unwrap()).1 {
            a
        } else {
            panic!("Not string value")
        };

        assert_eq!(value, *b);
    }

    #[test]
    fn test_load_vector_data() {
        let dir = env::temp_dir();
        let path_str = dir.to_str().unwrap().to_string() + &"/vector_data.txt".to_string();
        let path = dir.to_str().unwrap().to_string() + &"/vector_data.txt".to_string();

        let mut file = File::create(path).expect("Not file created");

        writeln!(file, "Daniela;|LISTA|;0;buen,dia").expect("Not file write");
        let mut data_storage = DataStorage::new();
        data_storage.load_data(&path_str);

        let key = String::from("Daniela");
        let first_value = String::from("buen");

        let read = data_storage.read();

        let b = if let Value::Vec(a) = &(*read.get(&key).unwrap()).1 {
            a
        } else {
            panic!("Not vector value")
        };

        assert_eq!(first_value, b[0]);
    }

    #[test]
    fn test_load_set_data() {
        let dir = env::temp_dir();
        let path_str = dir.to_str().unwrap().to_string() + &"/set_data.txt".to_string();
        let path = dir.to_str().unwrap().to_string() + &"/set_data.txt".to_string();

        let mut file = File::create(path).expect("Not file created");

        writeln!(file, "Daniela;|SET|;0;buen,dia").expect("Not file write");
        let mut data_storage = DataStorage::new();
        data_storage.load_data(&path_str);

        let key = String::from("Daniela");
        let first_value = String::from("buen");

        let read = data_storage.read();

        let b = if let Value::HashSet(a) = &(*read.get(&key).unwrap()).1 {
            a
        } else {
            panic!("Not set value")
        };

        assert!(b.contains(&first_value));
    }

    #[test]
    fn test_add_string_data() {
        let mut data_storage = DataStorage::new();
        let key = String::from("Daniela");
        let value = String::from("hola");

        data_storage.add_key_value(&key, Value::String(value));
        let read = data_storage.read();

        let b = if let Value::String(a) = &(*read.get(&key).unwrap()).1 {
            a
        } else {
            panic!("Not string value")
        };

        assert_eq!("hola", *b);
    }

    #[test]
    fn test_add_vector_data() {
        let mut data_storage = DataStorage::new();
        let key = String::from("Daniela");
        let value = vec!["a".to_string(), "b".to_string()];

        data_storage.add_key_value(&key, Value::Vec(value));
        let read = data_storage.read();

        let b = if let Value::Vec(a) = &(*read.get(&key).unwrap()).1 {
            a
        } else {
            panic!("Not string value")
        };

        assert_eq!(vec!["a", "b"], *b);
    }

    #[test]
    fn test_add_set_data() {
        let mut data_storage = DataStorage::new();
        let key = String::from("Daniela");
        let value: HashSet<String> = vec!["a".to_string(), "b".to_string()].into_iter().collect();

        data_storage.add_key_value(&key, Value::HashSet(value));
        let read = data_storage.read();

        let b = if let Value::HashSet(a) = &(*read.get(&key).unwrap()).1 {
            a
        } else {
            panic!("Not string value")
        };

        let a: HashSet<String> = vec!["a".to_string(), "b".to_string()].into_iter().collect();

        assert_eq!(a, *b);
    }
}
