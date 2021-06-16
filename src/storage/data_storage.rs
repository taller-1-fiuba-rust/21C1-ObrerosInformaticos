use crate::storage::entry::Entry;
use crate::storage::parser;
use crate::storage::SafeDataStorage;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::RwLock;
use std::sync::RwLockReadGuard;
use std::sync::{Arc, RwLockWriteGuard};
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

/// Enumeration value. Contains all supported data types
/// for the DataStorage.
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

/// Struct DataStorage. It is composed of a
/// HashMap which stores the information of the program.
/// Structure protected by a RwLock.
pub struct DataStorage {
    data: SafeDataStorage,
}

/// Implementation of the DataStorage structure.
#[allow(dead_code)]
impl DataStorage {
    /// Create the DataStorage structure.
    pub fn new() -> Self {
        DataStorage {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Given a filename load into the database
    /// the information contained.
    /// PRE: The file must have the supported structure
    /// for data loading and structure must be found
    /// previously created.
    /// POST: DataStorage is loaded with the data
    /// that contained the file.
    pub fn load_data(&self, file: &str) -> Result<(), &'static str> {
        let mut lock = self.data.write().ok().ok_or("Failed to lock database")?;
        match parser::parse_data(file, &mut lock) {
            Ok(_s) => Ok(()),
            Err(_i) => Err("Could not parse the file"),
        }
    }

    /// Given a file name, save the data of the
    /// database in it.
    /// PRE: The DataStorage structure must be created.
    /// POST: The file contains the information that had
    /// in the structure.
    pub fn save_data(&self, file: &str) -> Result<(), &'static str> {
        let lock = self.data.read().ok().ok_or("Failed to lock database")?;
        parser::store_data(file, &lock);
        Ok(())
    }

    /// Given a key and a value, it stores them in the database.
    /// PRE: The DataStorage structure must be created.
    /// POST: The key is stored in the structure with its value
    /// corresponding and with expiration time 0 given that the
    /// default keys never expire.
    pub fn set(&self, key: &str, value: Value) -> Result<(), &'static str> {
        let mut lock = self.data.write().ok().ok_or("Failed to lock database")?;
        self.do_set(&mut lock, key, value)?;
        Ok(())
    }

    fn do_set(
        &self,
        lock: &mut RwLockWriteGuard<HashMap<String, Entry>>,
        key: &str,
        value: Value,
    ) -> Result<(), &'static str> {
        let copy_key = key.to_string();
        match value {
            Value::String(s) => lock.insert(copy_key, Entry::new(now()?, None, Value::String(s))),
            Value::Vec(i) => lock.insert(copy_key, Entry::new(now()?, None, Value::Vec(i))),
            Value::HashSet(j) => lock.insert(copy_key, Entry::new(now()?, None, Value::HashSet(j))),
        };
        Ok(())
    }

    /// Set multiple keys at once
    pub fn set_multiple(&self, keys: Vec<String>, values: Vec<Value>) -> Result<(), &'static str> {
        let mut lock = self.data.write().ok().ok_or("Failed to lock database")?;
        for (key, value) in keys.iter().zip(values) {
            self.do_set(&mut lock, key, value)?;
        }
        Ok(())
    }

    ///Append the value at the end of the string if key already exists and is a string
    ///If key does not exist it is created and set as an empty string.
    pub fn append(&self, key: String, value: String) -> Result<usize, &'static str> {
        let mut lock = self.data.write().ok().ok_or("Failed to lock database")?;

        if lock.contains_key(&key) {
            let entry: &mut Entry = lock.get_mut(&key).unwrap();

            match entry.value() {
                Ok(val) => match val {
                    Value::String(s) => {
                        let new_string = s + &value;
                        let length = new_string.len();
                        entry.update_value(Value::String(new_string))?;
                        Ok(length)
                    }
                    Value::Vec(_i) => Err("Value must be a string not a vector"),
                    Value::HashSet(_j) => Err("Value must be a string not a set"),
                },
                Err(_s) => {
                    self.delete_key(&key)?;
                    let value_copy = value.clone();
                    match self.do_set(&mut lock, &key, Value::String(value_copy)) {
                        Ok(_s) => Ok(value.len()),
                        Err(_i) => Err("String value not created"),
                    }
                }
            }
        } else {
            let value_copy = value.clone();
            match self.do_set(&mut lock, &key, Value::String(value_copy)) {
                Ok(_s) => Ok(value.len()),
                Err(_i) => Err("String value not created"),
            }
        }
    }

    /// Remove the key with its corresponding value from the structure.
    /// PRE: The DataStorage structure must be created.
    /// POST: The key is removed and its corresponding value. In case
    /// if the key is not in the structure, an error is thrown.
    pub fn delete_key(&self, key: &str) -> Result<(), &'static str> {
        let mut lock = self.data.write().ok().ok_or("Failed to lock database")?;
        match lock.remove(key) {
            Some(_a) => Ok(()),
            None => Err("Not key in HashMap"),
        }
    }

    /// Returns OK if the key exists in the database and error otherwise.
    pub fn exists_key(&self, key: &str) -> Result<(), &'static str> {
        let lock = self.data.read().ok().ok_or("Failed to lock database")?;
        if lock.contains_key(key) {
            Ok(())
        } else {
            Err("Not key in HashMap")
        }
    }

    /// Returns a read reference for the DataStorage structure.
    pub fn read(&self) -> RwLockReadGuard<'_, HashMap<String, Entry>> {
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

    /// Returns Ok(Some(entryF)) for a specified key
    /// Returns Ok(None) if the key has expired
    /// Returns Err() if theres no value for that key
    pub fn get_entry(&self, key: &str) -> Result<Option<Entry>, &'static str> {
        let lock = self.data.read().ok().ok_or("Failed to lock database")?;

        if lock.contains_key(key) {
            let entry: &Entry = lock.get(key).unwrap();
            let key_exp = entry.key_expiration();
            let entry_cpy = entry.clone();

            if key_exp != Ok(None) {
                let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
                if key_exp.unwrap().unwrap() > now {
                    drop(lock);
                    self.modify_last_key_access(&key, now).unwrap();
                    return Ok(Some(entry_cpy));
                }
                // Key has expired, we should delete it
                drop(lock);
                self.delete_key(key).unwrap();
                return Ok(None);
            }
            Ok(Some(entry_cpy))
        } else {
            Err("No value for that key")
        }
    }

    /// Returns a tuple of expiration and value.
    pub fn get_with_expiration(&self, key: &str) -> Option<(Option<Duration>, Value)> {
        let lock = self.data.read().ok()?;

        if lock.contains_key(key) {
            let result = lock.get(key).unwrap();
            let key_exp = result.key_expiration();

            if key_exp != Ok(None) {
                match result.value() {
                    Ok(value) => {
                        drop(lock);
                        self.modify_last_key_access(&key, now().ok()?).unwrap();
                        return Some((key_exp.unwrap(), value));
                    }
                    Err(_s) => {
                        // Key has expired, we should delete it
                        drop(lock);
                        self.delete_key(key).unwrap();
                        return None;
                    }
                }
            }
            return Some((None, result.value().unwrap()));
        }

        None
    }

    pub fn getset(&self, key: &str, new_value: Value) -> Result<String, &'static str> {
        let mut lock = self.data.write().ok().ok_or("Failed to lock database")?;

        match lock.get(key) {
            Some(entry) => match entry.value() {
                Ok(value) => match value {
                    Value::String(old_value) => {
                        self.do_set(&mut lock, key, new_value)?;
                        drop(lock);
                        Ok(old_value)
                    }
                    Value::Vec(_) => {
                        Err("WRONGTYPE Operation against a key holding the wrong kind of value")
                    }
                    Value::HashSet(_) => {
                        Err("WRONGTYPE Operation against a key holding the wrong kind of value")
                    }
                },
                Err(_s) => {
                    self.delete_key(key)?;
                    Ok("nil".to_string())
                }
            },
            None => Ok("nil".to_string()),
        }
    }

    /// Renames a key and fails if it does not exist.
    pub fn rename(&self, src: &str, dst: &str) -> Result<(), &'static str> {
        let lock = self.data.read().ok().ok_or("Failed to lock database")?;
        if lock.contains_key(src) {
            let result = lock.get(src).unwrap();
            match result.value() {
                Ok(_) => {
                    let moved_duration = result.key_expiration().unwrap();
                    let moved_val = result.value().unwrap();
                    drop(lock);
                    self.set(dst, moved_val)?;
                    self.set_expiration_to_key(moved_duration, dst)?;
                    self.delete_key(src)?;
                    Ok(())
                }
                Err(_s) => {
                    self.delete_key(src)?;
                    Err("No such key")
                }
            }
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
        self.set(key, value)?;
        self.set_expiration_to_key(Some(expiration_time_since_unix_epoch), key)?;
        Ok(())
    }

    /// Set an expiration to a given key.
    /// PRE: The DataStorage structure must be created.
    /// POST: The key has a set expiration time. In case
    /// if the key does not exist in the structure, an error is thrown.
    pub fn set_expiration_to_key(
        &self,
        expiration_time_since_unix_epoch: Option<Duration>,
        key: &str,
    ) -> Result<u64, &'static str> {
        let mut lock = self.data.write().ok().ok_or("Failed to lock database")?;
        let copy_key = key.to_string();

        if lock.contains_key(&copy_key) {
            lock.get_mut(&copy_key)
                .unwrap()
                .set_key_expiration(expiration_time_since_unix_epoch);
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
        result
    }

    pub fn modify_last_key_access(
        &self,
        key: &str,
        last_access_since_unix_epoch: Duration,
    ) -> Result<(), &'static str> {
        let mut lock = self.data.write().ok().ok_or("Failed to lock database")?;
        let copy_key = key.to_string();

        if lock.contains_key(&copy_key) {
            let result = lock
                .get_mut(&copy_key)
                .unwrap()
                .set_last_access(last_access_since_unix_epoch);
            match result {
                Ok(_s) => return Ok(()),
                Err(_s) => {
                    self.delete_key(&key)?;
                    return Err("last access not modify not existing key");
                }
            }
        }
        Err("last access not modify")
    }

    pub fn decrement_value(&self, key: String, numeric_value: i64) -> Result<i64, &'static str> {
        let mut lock = self.data.write().ok().ok_or("Failed to lock database")?;

        if lock.contains_key(&key) {
            let entry: &mut Entry = lock.get_mut(&key).unwrap();
            match entry.value() {
                Ok(value) => match value {
                    Value::String(s) => match s.parse::<i64>() {
                        Ok(number) => {
                            let new_value = number - numeric_value;
                            entry.update_value(Value::String(new_value.to_string()))?;
                            Ok(number - numeric_value)
                        }
                        Err(_j) => Err("Cant decrement a value to a not integer value"),
                    },
                    Value::Vec(_i) => Err("Cant decrement a value to a vector"),
                    Value::HashSet(_j) => Err("Cant decrement a value to a set"),
                },
                Err(_s) => {
                    self.delete_key(&key)?;
                    let negative_value = 0 - numeric_value;
                    self.do_set(&mut lock, &key, Value::String(negative_value.to_string()))?;
                    Ok(0 - numeric_value)
                }
            }
        } else {
            let negative_value = 0 - numeric_value;
            self.do_set(&mut lock, &key, Value::String(negative_value.to_string()))?;
            Ok(0 - numeric_value)
        }
    }

    pub fn get_string_value(&self, key: String) -> Result<Option<String>, &'static str> {
        let lock = self.data.read().ok().ok_or("Failed to lock database")?;

        if lock.contains_key(&key) {
            let entry = lock.get(&key).unwrap();

            match entry.value() {
                Ok(value) => match value {
                    Value::String(string_value) => Ok(Some(string_value)),
                    Value::Vec(_i) => Err("value not a string"),
                    Value::HashSet(_j) => Err("value not a string"),
                },
                Err(_s) => {
                    self.delete_key(&key)?;
                    Ok(None)
                }
            }
        } else {
            Ok(None)
        }
    }
}

fn now() -> Result<Duration, &'static str> {
    let _now = SystemTime::now().duration_since(UNIX_EPOCH);

    match _now {
        Ok(now) => Ok(now),
        Err(_) => Err("Cannot get actual timestamp"),
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

        data_storage.set(&key, Value::String(value)).unwrap();
        data_storage.delete_key(&key).unwrap();

        let read = data_storage.read();

        if let Value::String(a) = read.get(&key).unwrap().value().unwrap() {
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

        data_storage.set(&key, Value::String(value)).unwrap();

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
        let key_expiration: &Option<Duration> = &read.get(&key).unwrap().key_expiration().unwrap();

        assert_eq!(expiration_time.as_secs(), key_expiration.unwrap().as_secs());
    }

    #[test]
    fn test_load_string_data() {
        let dir = env::temp_dir();
        let path_str = dir.to_str().unwrap().to_string() + &"/string_data.txt".to_string();
        let path = dir.to_str().unwrap().to_string() + &"/string_data.txt".to_string();

        let mut file = File::create(path).expect("Not file created");

        writeln!(file, "Daniela;|STRING|;12356;0;hola").expect("Not file write");
        let data_storage = DataStorage::new();
        data_storage.load_data(&path_str).unwrap();

        let key = String::from("Daniela");
        let value = String::from("hola");

        let read = data_storage.read();

        let b = if let Value::String(a) = read.get(&key).unwrap().value().unwrap() {
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

        writeln!(file, "Daniela;|LISTA|;12345;0;buen,dia").expect("Not file write");
        let data_storage = DataStorage::new();
        data_storage.load_data(&path_str).unwrap();

        let key = String::from("Daniela");
        let first_value = String::from("buen");

        let read = data_storage.read();

        let b = if let Value::Vec(a) = read.get(&key).unwrap().value().unwrap() {
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

        writeln!(file, "Daniela;|SET|;12356;0;buen,dia").expect("Not file write");
        let data_storage = DataStorage::new();
        data_storage.load_data(&path_str).unwrap();

        let key = String::from("Daniela");
        let first_value = String::from("buen");

        let read = data_storage.read();

        let b = if let Value::HashSet(a) = read.get(&key).unwrap().value().unwrap() {
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

        data_storage.set(&key, Value::String(value)).unwrap();
        let read = data_storage.read();

        let b = if let Value::String(a) = read.get(&key).unwrap().value().unwrap() {
            a
        } else {
            panic!("Not string value")
        };

        assert_eq!("hola", b);
    }

    #[test]
    fn test_add_vector_data() {
        let data_storage = DataStorage::new();
        let key = String::from("Daniela");
        let value = vec!["a".to_string(), "b".to_string()];

        data_storage.set(&key, Value::Vec(value)).unwrap();
        let read = data_storage.read();

        let b = if let Value::Vec(a) = read.get(&key).unwrap().value().unwrap() {
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

        data_storage.set(&key, Value::HashSet(value)).unwrap();
        let read = data_storage.read();

        let b = if let Value::HashSet(a) = read.get(&key).unwrap().value().unwrap() {
            a
        } else {
            panic!("Not string value")
        };

        let a: HashSet<String> = vec!["a".to_string(), "b".to_string()].into_iter().collect();

        assert_eq!(a, b);
    }
}
