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
#[derive(Clone)]
pub enum Value {
    String(String),
    Vec(Vec<String>),
    HashSet(HashSet<String>),
}
#[allow(dead_code)]
impl Value {
    pub fn string(&self) -> Result<String, &'static str> {
        match self {
            Value::String(s) => Ok(s.clone()),
            _ => Err("Failed to cast Value to string"),
        }
    }

    pub fn array(&self) -> Result<Vec<String>, &'static str> {
        match self {
            Value::Vec(v) => Ok(v.clone()),
            _ => Err("Failed to cast Value to string"),
        }
    }

    pub fn set(&self) -> Result<HashSet<String>, &'static str> {
        match self {
            Value::HashSet(s) => Ok(s.clone()),
            _ => Err("Failed to cast Value to string"),
        }
    }
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
    pub fn add_key_value(&self, key: &str, value: Value) {
        let mut lock = self.data.write().unwrap();
        let copy_key = key.to_string();

        match value {
            Value::String(s) => lock.insert(copy_key, (None, Value::String(s))),
            Value::Vec(i) => lock.insert(copy_key, (None, Value::Vec(i))),
            Value::HashSet(j) => lock.insert(copy_key, (None, Value::HashSet(j))),
        };
    }

    ///Elimina la clave con su correspondiente valor de la estructura.
    ///PRE: La estuctura DataStorage debe estar creada.
    ///POST: La clave es eliminada y su correspondiente valor. En caso
    ///de no estar la clave en la estructura se lanza error.
    pub fn delete_key(&self, key: &str) -> Result<(), &'static str> {
        let mut lock = self.data.write().unwrap();
        match lock.remove(key) {
            Some(_a) => Ok(()),
            None => Err("Not key in HashMap"),
        }
    }

    ///Devuelve OK si la clave existe en la base de datos y error en caso contrario.
    pub fn exists_key(&self, key: &str) -> Result<(), &'static str> {
        let lock = self.data.read().unwrap();
        if lock.contains_key(key) {
            Ok(())
        } else {
            Err("Not key in HashMap")
        }
    }

    ///Devuelve una referencia de lectura para la estructura DataStorage.
    pub fn read(&self) -> RwLockReadGuard<'_, HashMap<String, (Option<Duration>, Value)>> {
        self.data.read().unwrap()
    }

    /// Returns a copy of the value at key or none if it doesnt exist.
    pub fn get(&self, key: &str) -> Option<Value> {
        let result = self.get_with_expiration(key);
        if let Some((_, value)) = result {
            Some(value)
        } else {
            None
        }
    }

    /// Returns a tuple of expiration and value.
    pub fn get_with_expiration(&self, key: &str) -> Option<(Option<Duration>, Value)> {
        let lock = self.data.read().ok()?;
        let result = lock.get(key);
        if let Some((duration, val)) = result {
            if let Some(seconds) = duration {
                let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
                if seconds > &now {
                    return Some((Some(*seconds), val.clone()));
                }
                // Key has expired, we should delete it
                drop(lock);
                self.delete_key(key).unwrap();
                return None;
            }
            return Some((None, val.clone()));
        }
        None
    }

    /// Renames a key and fails if it does not exist
    pub fn rename(&self, src: &str, dst: &str) -> Result<(), &'static str> {
        let lock = self.data.read().ok().ok_or("Failed to lock database")?;
        let result = lock.get(src);
        if let Some((duration, val)) = result {
            let moved_duration = *duration;
            let moved_val = val.clone();
            drop(lock);
            self.add_key_value(dst, moved_val);
            self.set_expiration_to_key(moved_duration, dst)?;
            self.delete_key(src)?;
            Ok(())
        } else {
            Err("No such key")
        }
    }

    /// Adds a key into the db with the specified expiration date
    pub fn add_with_expiration(
        &self,
        key: &str,
        value: Value,
        expiration_time_since_unix_epoch: Duration,
    ) -> Result<(), &'static str> {
        self.add_key_value(key, value);
        self.set_expiration_to_key(Some(expiration_time_since_unix_epoch), key)?;
        Ok(())
    }

    ///Setea una expiracion a una clave dada.
    ///PRE: La estructura DataStorage debe estar creada.
    ///POST: La clave queda con un tiempo de expiracion seteado. En caso
    ///de no existir la clave en la estructura se lanza un error.
    pub fn set_expiration_to_key(
        &self,
        expiration_time_since_unix_epoch: Option<Duration>,
        key: &str,
    ) -> Result<u64, &'static str> {
        let mut lock = self.data.write().unwrap();
        let copy_key = key.to_string();

        if lock.contains_key(&copy_key) {
            lock.get_mut(&copy_key).unwrap().0 = expiration_time_since_unix_epoch;
            Ok(1)
        } else {
            Err("Key not found in DataStorage")
        }
    }

    pub fn get_keys(&self) -> Vec<String> {
        let lock = self.read();
        let mut result = Vec::new();
        for key in lock.keys() {
            result.push(key.clone());
        }
        return result;
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
        let data_storage = DataStorage::new();
        let key = String::from("Daniela");
        let value = String::from("hola");

        data_storage.add_key_value(&key, Value::String(value));
        data_storage.delete_key(&key).unwrap();

        let read = data_storage.read();

        if let Value::String(a) = &(*read.get(&key).unwrap()).1 {
            a
        } else {
            panic!("Value not found in storage")
        };
    }

    #[test]
    fn test_set_expiration_to_key() {
        let data_storage = DataStorage::new();
        let key = String::from("Daniela");
        let value = String::from("hola");
        let duration = Duration::from_secs(5);

        data_storage.add_key_value(&key, Value::String(value));

        let expiration_time = SystemTime::now()
            .checked_add(duration)
            .unwrap()
            .duration_since(UNIX_EPOCH)
            .unwrap();
        let _result = match data_storage.set_expiration_to_key(Some(expiration_time), &key) {
            Ok(s) => s,
            Err(_s) => panic!("Key expiration cant be set"),
        };

        let read = data_storage.read();
        let key_expiration: &Option<Duration> = &(*read.get(&key).unwrap()).0;

        assert_eq!(expiration_time.as_secs(), key_expiration.unwrap().as_secs());
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
        let data_storage = DataStorage::new();
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
        let data_storage = DataStorage::new();
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
        let data_storage = DataStorage::new();
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
