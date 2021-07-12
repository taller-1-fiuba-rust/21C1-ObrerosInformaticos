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
use std::usize;

/// Enumeration value. Contains all supported data types
/// for the DataStorage.
#[derive(Clone)]
pub enum Value {
    String(String),
    Vec(Vec<String>),
    HashSet(HashSet<String>),
}

#[allow(dead_code)]
///Implementation of the Value structure.
impl Value {
    /// Given a possible Value String, it analyzes if the value
    /// can be obtained as a string and returns it, if it is another type of data,
    /// it returns an error.
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use proyecto_taller_1::storage::data_storage::Value::String;
    /// use proyecto_taller_1::storage::data_storage::Value;
    /// let value = Value::String("hola".to_string());
    /// value.string();
    /// ```
    ///
    pub fn string(&self) -> Result<String, &'static str> {
        match self {
            Value::String(s) => Ok(s.clone()),
            _ => Err("Failed to cast Value to string"),
        }
    }

    /// Given a possible Value Array, it analyzes if the value
    /// can be obtained as an array and returns it, if it is another type of data,
    /// it returns an error.
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use proyecto_taller_1::storage::data_storage::Value::Vec;
    /// use proyecto_taller_1::storage::data_storage::Value;
    /// let value = Value::Vec(["hola".to_string()].to_vec());
    /// value.array();
    /// ```
    ///
    pub fn array(&self) -> Result<Vec<String>, &'static str> {
        match self {
            Value::Vec(v) => Ok(v.clone()),
            _ => Err("Failed to cast Value to string"),
        }
    }

    /// Given a possible Value HashSet, it analyzes if the value
    /// can be obtained as a set and returns it, if it is another type of data,
    /// it returns an error.
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use proyecto_taller_1::storage::data_storage::Value::Vec;
    /// use proyecto_taller_1::storage::data_storage::Value;
    /// use std::collections::HashSet;
    /// let set = HashSet::new();
    /// let value = Value::HashSet(set);
    /// value.set();
    /// ```
    ///
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
#[allow(clippy::new_without_default)]
impl DataStorage {
    /// Create the DataStorage structure. Initially the database will be instantiated empty
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use proyecto_taller_1::storage::data_storage::DataStorage;
    /// let data = DataStorage::new();
    /// ```
    ///
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
    /// # Arguments
    ///
    /// * `file` - A string slice that holds the name of the file to use.
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use proyecto_taller_1::storage::data_storage::DataStorage;
    /// let data = DataStorage::new();
    /// data.load_data(&"new_file.txt");
    /// ```
    ///
    pub fn load_data(&self, file: &str) -> Result<(), &'static str> {
        let mut lock = self.data.write().ok().ok_or("Failed to lock database")?;
        match parser::parse_data(file, &mut lock) {
            Ok(_s) => Ok(()),
            Err(_i) => Err("Could not parse the file"),
        }
    }

    /// Given a file name, save the data of the
    /// database in it.
    /// The DataStorage structure must be created.
    /// At the end, the file contains the information that had
    /// in the structure.
    /// # Arguments
    ///
    /// * `file` - A string slice that holds the name of the file to use.
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use proyecto_taller_1::storage::data_storage::DataStorage;
    /// let data = DataStorage::new();
    /// data.save_data(&"file.txt");
    /// ```
    ///
    pub fn save_data(&self, file: &str) -> Result<(), &'static str> {
        let lock = self.data.read().ok().ok_or("Failed to lock database")?;
        parser::store_data(file, &lock);
        Ok(())
    }

    /// Given a key and a value, it stores them in the database.
    /// The DataStorage structure must be created.
    /// At the end, the key is stored in the structure with its value
    /// corresponding and with expiration time 0 given that the
    /// default keys never expire.
    /// # Arguments
    ///
    /// * `key` - A string slice that holds the name of the key to store.
    /// * `value` - A Value slice that holds the value to store.
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use proyecto_taller_1::storage::data_storage::DataStorage;
    /// use proyecto_taller_1::storage::data_storage::Value::String;
    /// use proyecto_taller_1::storage::data_storage::Value;
    /// let data = DataStorage::new();
    /// data.set(&"key", Value::String("hola".to_string()));
    /// ```
    ///
    pub fn set(&self, key: &str, value: Value) -> Result<(), &'static str> {
        let mut lock = self.data.write().ok().ok_or("Failed to lock database")?;
        self.do_set(&mut lock, key, value)?;
        Ok(())
    }

    /// Safety function used to set a key and a value in the data base.
    /// # Arguments
    ///
    /// * `lock` - A RwLockWriteGuard slice that holds lock to the lock to safely access the structure.
    /// * `key` - A string slice that holds the name of the key to store.
    /// * `value` - A Value slice that holds the value to store.
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

    /// Set multiple keys at once in the data structure.
    /// # Arguments
    ///
    /// * `keys` - A Vector of strings that holds the name of the keys to store.
    /// * `values` - A Vector of Values that holds the values to store.
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use proyecto_taller_1::storage::data_storage::DataStorage;
    /// use proyecto_taller_1::storage::data_storage::Value::String;
    /// use proyecto_taller_1::storage::data_storage::Value;
    /// let data = DataStorage::new();
    /// let values = vec![Value::String("value1".to_string()), Value::String("value2".to_string())];
    /// let keys = vec!["key1".to_string(), "key2".to_string()];
    /// data.set_multiple(keys, values);
    /// ```
    ///
    pub fn set_multiple(&self, keys: Vec<String>, values: Vec<Value>) -> Result<(), &'static str> {
        let mut lock = self.data.write().ok().ok_or("Failed to lock database")?;
        for (key, value) in keys.iter().zip(values) {
            self.do_set(&mut lock, key, value)?;
        }
        Ok(())
    }

    /// Remove the key with its corresponding value from the structure.
    /// The DataStorage structure must be created.
    /// At the end, the key is removed and its corresponding value. In case
    /// if the key is not in the structure, an error is thrown.
    /// # Arguments
    ///
    /// * `key` - A strings that holds the name of the key to delete.
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use proyecto_taller_1::storage::data_storage::DataStorage;
    /// let data = DataStorage::new();
    /// data.delete_key(&"key".to_string());
    /// ```
    ///
    pub fn delete_key(&self, key: &str) -> Result<(), &'static str> {
        let mut lock = self.data.write().ok().ok_or("Failed to lock database")?;
        self.do_delete_key(&mut lock, key)
    }

    /// Remove the key with its corresponding value from the structure.
    /// PRE: The DataStorage structure must be created.
    /// POST: The key is removed and its corresponding value. In case
    /// if the key is not in the structure, an error is thrown.
    fn do_delete_key<'i>(
        &self,
        lock: &'i mut RwLockWriteGuard<HashMap<String, Entry>>,
        key: &str,
    ) -> Result<(), &'static str> {
        match lock.remove(key) {
            Some(_a) => Ok(()),
            None => Err("Not key in HashMap"),
        }
    }

    ///Delete all the keys of the currently selected DB.
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use proyecto_taller_1::storage::data_storage::DataStorage;
    /// let data = DataStorage::new();
    /// data.delete_all();
    /// ```
    ///
    pub fn delete_all(&self) -> Result<(), &'static str> {
        let mut lock = self.data.write().ok().ok_or("Failed to lock database")?;
        lock.clear();
        Ok(())
    }

    ///Return TRUE if the storage is empty or FALSE if not.
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use proyecto_taller_1::storage::data_storage::DataStorage;
    /// let data = DataStorage::new();
    /// let empty = data.is_empty();
    /// ```
    ///
    pub fn is_empty(&self) -> bool {
        let lock = self.data.read().unwrap();
        lock.is_empty()
    }

    /// Returns OK if the key exists in the database and error otherwise.
    /// # Arguments
    ///
    /// * `key` - A string that holds the name of the key to get.
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use proyecto_taller_1::storage::data_storage::DataStorage;
    /// let data = DataStorage::new();
    /// let exists = data.exists_key(&"key".to_string());
    /// ```
    ///
    pub fn exists_key(&self, key: &str) -> Result<(), &'static str> {
        let value = self.get(&key);
        match value {
            Some(_) => Ok(()),
            None => Err("Not key in HashMap"),
        }
    }

    /// Returns the number of elements in the database.
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use proyecto_taller_1::storage::data_storage::DataStorage;
    /// let data = DataStorage::new();
    /// let len = data.len();
    /// ```
    ///
    pub fn len(&self) -> Result<usize, &'static str> {
        let lock = self.data.read().ok().ok_or("Failed to lock database")?;
        let mut count = 0;
        for entry in lock.values() {
            match entry.value() {
                Ok(_) => count += 1,
                Err(_) => continue,
            };
        }
        Ok(count)
    }

    /// Returns a read reference for the DataStorage structure.
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use proyecto_taller_1::storage::data_storage::DataStorage;
    /// let data = DataStorage::new();
    /// let read_data = data.read();
    /// ```
    ///
    pub fn read(&self) -> RwLockReadGuard<'_, HashMap<String, Entry>> {
        self.data.read().unwrap()
    }

    /// Returns a copy of the value at key or none if it doesnt exist.
    /// # Arguments
    ///
    /// * `key` - A string that holds the name of the key to get.
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use proyecto_taller_1::storage::data_storage::DataStorage;
    /// let data = DataStorage::new();
    /// let key = data.get(&"key".to_string());
    /// ```
    ///
    pub fn get(&self, key: &str) -> Option<Value> {
        let result = self.get_with_expiration(key);
        if let Some((_, value)) = result {
            Some(value)
        } else {
            None
        }
    }

    /// Given a key it returns a tuple of expiration and value.
    /// # Arguments
    ///
    /// * `key` - A string that holds the name of the key to get.
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use proyecto_taller_1::storage::data_storage::DataStorage;
    /// let data = DataStorage::new();
    /// let value = data.get_with_expiration(&"key".to_string());
    /// ```
    ///
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

    /// Give a key it returns Ok(Some(entryF)) for a specified key,
    /// Returns Ok(None) if the key has expired
    /// Returns Err() if theres no value for that key
    /// # Arguments
    ///
    /// * `key` - A string that holds the name of the key to get.
    /// * `lock` - A RwLockWriteGuard slice that holds lock to the lock to safely access the structure.
    ///
    pub fn get_entry<'i>(
        &self,
        key: &str,
        lock: &'i mut RwLockWriteGuard<HashMap<String, Entry>>,
    ) -> Result<Option<&'i mut Entry>, &'static str> {
        if lock.contains_key(key) {
            let entry: &Entry = lock.get(key).unwrap();
            let key_exp = entry.key_expiration();

            let res = match key_exp {
                Ok(expiration) => match expiration {
                    Some(exp) => {
                        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
                        if exp > now {
                            self.do_modify_last_key_access(lock, &key, now).unwrap();
                            Ok(Some(()))
                        } else {
                            self.do_delete_key(lock, key)?;
                            Ok(None)
                        }
                    }
                    None => Ok(Some(())),
                },
                Err(_) => Err("No value for that key"),
            };
            match res {
                Ok(v) => match v {
                    Some(_) => Ok(Some(lock.get_mut(key).unwrap())),
                    None => Ok(None),
                },
                Err(e) => Err(e),
            }
        } else {
            Err("No value for that key")
        }
    }

    ///Removes and returns the first elements of the list stored at key.
    /// # Arguments
    ///
    /// * `key` - A string that holds the name of the key to delete the first value in the list.
    /// * `count` - A usize slice.
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use proyecto_taller_1::storage::data_storage::DataStorage;
    /// let data = DataStorage::new();
    /// let value = data.lpop("key".to_string(), 0);
    /// ```
    ///
    pub fn lpop(&self, key: String, count: usize) -> Result<Vec<String>, &'static str> {
        let mut lock = self.data.write().ok().ok_or("Failed to lock database")?;
        let mut result = Vec::new();
        let _ = self.do_apply_vec(key, &mut lock, |list| {
            for _ in 0..count {
                if list.is_empty() {
                    break;
                }
                result.push(list.remove(0));
            }
        })?;
        Ok(result)
    }

    /// Applies a function to a list and returns its resulting length
    /// # Arguments
    ///
    /// * `key` - A string that holds the name of the key to delete.
    /// * `lock` - A RwLockWriteGuard slice that holds lock to the lock to safely access the structure.
    ///
    fn do_apply_vec<F: FnMut(&mut Vec<String>)>(
        &self,
        key: String,
        lock: &mut RwLockWriteGuard<HashMap<String, Entry>>,
        mut apply: F,
    ) -> Result<usize, &'static str> {
        let res_entry = self.get_entry(&key, lock);
        if res_entry.is_err() {
            return Ok(0);
        }
        match res_entry.unwrap() {
            Some(entry) => match entry.value() {
                Ok(val) => match val {
                    Value::String(_) => {
                        Err("WRONGTYPE Operation against a key holding the wrong kind of value")
                    }
                    Value::Vec(mut v) => {
                        apply(&mut v);
                        let len = v.len();
                        entry.update_value(Value::Vec(v))?;
                        Ok(len)
                    }
                    Value::HashSet(_) => {
                        Err("WRONGTYPE Operation against a key holding the wrong kind of value")
                    }
                },
                Err(_) => Ok(0),
            },
            None => Ok(0),
        }
    }

    /// Append the value at the end of the string if key already exists and is a string
    /// If key does not exist it is created and set as an empty string.
    /// # Arguments
    ///
    /// * `key` - A string that holds the name of the key to get and append the value.
    /// * `value` - A string to append in the current value for the given key.
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use proyecto_taller_1::storage::data_storage::DataStorage;
    /// let data = DataStorage::new();
    /// let value = data.append("key".to_string(), "new_value".to_string());
    /// ```
    ///
    pub fn append(&self, key: String, value: String) -> Result<usize, &'static str> {
        let mut lock = self.data.write().ok().ok_or("Failed to lock database")?;
        let res_entry = self.get_entry(&key, &mut lock);

        match res_entry {
            Ok(opt_entry) => match opt_entry {
                Some(entry) => {
                    let old_value = entry.value().unwrap();
                    match old_value {
                        Value::String(s) => {
                            let new_string = s + &value;
                            let length = new_string.len();
                            entry.update_value(Value::String(new_string))?;
                            Ok(length)
                        }
                        Value::Vec(_i) => Err("Value must be a string not a vector"),
                        Value::HashSet(_j) => Err("Value must be a string not a set"),
                    }
                }
                None => {
                    let value_copy = value.clone();
                    match self.do_set(&mut lock, &key, Value::String(value_copy)) {
                        Ok(_s) => Ok(value.len()),
                        Err(_i) => Err("String value not created"),
                    }
                }
            },
            Err(_) => {
                let value_copy = value.clone();
                match self.do_set(&mut lock, &key, Value::String(value_copy)) {
                    Ok(_s) => Ok(value.len()),
                    Err(_i) => Err("String value not created"),
                }
            }
        }
    }

    ///Given a key returns the string value.
    ///In case there is another type of data stored in that key, an error is returned.
    /// # Arguments
    ///
    /// * `key` - A string that holds the name of the key to get.
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use proyecto_taller_1::storage::data_storage::DataStorage;
    /// let data = DataStorage::new();
    /// let value = data.get_string_value("key".to_string());
    /// ```
    ///
    pub fn get_string_value(&self, key: String) -> Result<Option<String>, &'static str> {
        let value = self.get(&key);

        match value {
            Some(val) => match val {
                Value::String(string_value) => Ok(Some(string_value)),
                Value::Vec(_i) => Err("value not a string"),
                Value::HashSet(_j) => Err("value not a string"),
            },
            None => Ok(None),
        }
    }

    /// Atomically sets key to value and returns the old value stored at key.
    /// Returns an error when key exists but does not hold a string value.
    /// Any previous time to live associated with the key is discarded on successful SET operation.
    /// # Arguments
    ///
    /// * `key` - A string that holds the name of the key to get and change the value.
    /// * `new_value` - A string that holds the new value to set.
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use proyecto_taller_1::storage::data_storage::DataStorage;
    /// use proyecto_taller_1::storage::data_storage::Value::String;
    /// use proyecto_taller_1::storage::data_storage::Value;
    /// let data = DataStorage::new();
    /// data.getset(&"key", Value::String("hola".to_string()));
    /// ```
    ///
    pub fn getset(&self, key: &str, new_value: Value) -> Result<String, &'static str> {
        let mut lock = self.data.write().ok().ok_or("Failed to lock database")?;
        let res_entry = self.get_entry(key, &mut lock);
        match res_entry {
            Ok(opt_entry) => match opt_entry {
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
                    Err(_) => Ok("nil".to_string()),
                },
                None => Ok("nil".to_string()),
            },
            Err(_) => Ok("nil".to_string()),
        }
    }

    /// Renames key to newkey.
    /// It returns an error when key does not exist.
    /// If newkey already exists it is overwritten
    /// when this happens RENAME executes an implicit DEL operation.
    /// # Arguments
    ///
    /// * `src` - A string that holds the name of the key to get and rename.
    /// * `Dst` - A string that holds the new name of the key to set.
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use proyecto_taller_1::storage::data_storage::DataStorage;
    /// let data = DataStorage::new();
    /// data.rename(&"key", &"new_key");
    /// ```
    ///
    pub fn rename(&self, src: &str, dst: &str) -> Result<(), &'static str> {
        let mut lock = self.data.write().ok().ok_or("Failed to lock database")?;
        let res_entry = self.get_entry(src, &mut lock);
        match res_entry {
            Ok(opt_entry) => match opt_entry {
                Some(entry) => match entry.value() {
                    Ok(_) => {
                        let moved_duration = entry.key_expiration().unwrap();
                        let moved_val = entry.value().unwrap();
                        drop(lock);
                        self.set(dst, moved_val)?;
                        self.set_expiration_to_key(moved_duration, dst)?;
                        self.delete_key(src)?;
                        Ok(())
                    }
                    Err(_) => Err("No such key"),
                },
                None => Err("No such key"),
            },
            Err(_) => Err("No such key"),
        }
    }

    /// Adds a key into the data base with the specified expiration date.
    /// # Arguments
    ///
    /// * `key` - A string that holds the name of the key to add.
    /// * `value` - A string that holds the value to set.
    /// * `expiration_time_since_unix_epoch` - A Duration that holds the expiration time of the key since unix epoch.
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use proyecto_taller_1::storage::data_storage::DataStorage;
    /// use proyecto_taller_1::storage::data_storage::Value::String;
    /// use proyecto_taller_1::storage::data_storage::Value;
    /// use std::time::{SystemTime, UNIX_EPOCH};
    /// let data = DataStorage::new();
    /// data.add_with_expiration(&"key", Value::String("hola".to_string()), SystemTime::now().duration_since(UNIX_EPOCH).unwrap());
    /// ```
    ///
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
    /// The DataStorage structure must be created.
    /// At the end, the key has a set expiration time. In case
    /// if the key does not exist in the structure, an error is thrown.
    /// # Arguments
    ///
    /// * `key` - A string that holds the name of the key to add the expiration.
    /// * `expiration_time_since_unix_epoch` - A Duration that holds the expiration time of the key since unix epoch.
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use proyecto_taller_1::storage::data_storage::DataStorage;
    /// use proyecto_taller_1::storage::data_storage::Value::String;
    /// use proyecto_taller_1::storage::data_storage::Value;
    /// use std::time::{SystemTime, UNIX_EPOCH};
    /// let data = DataStorage::new();
    /// data.set_expiration_to_key(Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap()), &"key");
    /// ```
    ///
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

    /// Return TRUE if the data base contains the key or FALSE otherwise.
    /// # Arguments
    ///
    /// * `key` - A string that holds the name of the key to add the expiration.
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use proyecto_taller_1::storage::data_storage::DataStorage;
    /// let data = DataStorage::new();
    /// data.contains_key("key".to_string());
    /// ```
    ///
    pub fn contains_key(&self, key: String) -> bool {
        let lock = self.read();
        lock.contains_key(&key)
    }

    /// Get a Vector with all the keys stores in the data base.
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use proyecto_taller_1::storage::data_storage::DataStorage;
    /// let data = DataStorage::new();
    /// let keys: Vec<String> = data.get_keys();
    /// ```
    ///
    pub fn get_keys(&self) -> Vec<String> {
        let lock = self.read();
        let mut result = Vec::new();
        for key in lock.keys() {
            result.push(key.clone());
        }
        result
    }

    ///Modify last key access if the key exist or is not expired.
    /// # Arguments
    ///
    /// * `key` - A string that holds the name of the key to modify last access.
    /// * `last_access_since_unix_epoch` - A Duration that holds the expiration time of the key since unix epoch.
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use proyecto_taller_1::storage::data_storage::DataStorage;
    /// use proyecto_taller_1::storage::data_storage::Value::String;
    /// use proyecto_taller_1::storage::data_storage::Value;
    /// use std::time::{SystemTime, UNIX_EPOCH};
    /// let data = DataStorage::new();
    /// data.modify_last_key_access(&"key", SystemTime::now().duration_since(UNIX_EPOCH).unwrap());
    /// ```
    ///
    pub fn modify_last_key_access(
        &self,
        key: &str,
        last_access_since_unix_epoch: Duration,
    ) -> Result<Duration, &'static str> {
        let mut lock = self.data.write().ok().ok_or("Failed to lock database")?;
        self.do_modify_last_key_access(&mut lock, key, last_access_since_unix_epoch)
    }

    ///Do modify last key access if the key exist or is not expired.
    fn do_modify_last_key_access<'i>(
        &self,
        lock: &'i mut RwLockWriteGuard<HashMap<String, Entry>>,
        key: &str,
        last_access_since_unix_epoch: Duration,
    ) -> Result<Duration, &'static str> {
        let copy_key = key.to_string();

        if lock.contains_key(&copy_key) {
            let previous_last_access = lock.get_mut(&copy_key).unwrap().last_access();
            let result = lock
                .get_mut(&copy_key)
                .unwrap()
                .set_last_access(last_access_since_unix_epoch);
            match previous_last_access {
                Ok(l_a) => match result {
                    Ok(_s) => return Ok(l_a),
                    Err(_s) => {
                        self.delete_key(&key)?;
                        return Err("last access not modify not existing key");
                    }
                },
                Err(_) => {
                    self.delete_key(&key)?;
                    return Err("last access not modify not existing key");
                }
            }
        }
        Err("last access not modify")
    }

    /// Increments the number stored at key by increment.
    /// If the key does not exist, it is set to 0 before performing the operation.
    /// An error is returned if the key contains a value of the wrong type or
    /// contains a string that can not be represented as integer.
    /// # Arguments
    ///
    /// * `key` - A string that holds the name of the key to increment the asociated value.
    /// * `numeric_value` - A i64 number that holds the number to add to the old value.
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use proyecto_taller_1::storage::data_storage::DataStorage;
    /// let data = DataStorage::new();
    /// data.increment_value("key".to_string(), 5);
    /// ```
    ///
    pub fn increment_value(&self, key: String, numeric_value: i64) -> Result<i64, &'static str> {
        let mut lock = self.data.write().ok().ok_or("Failed to lock database")?;
        let value = lock.get(&key);

        match value {
            Some(val) => match val.value()? {
                Value::String(s) => match s.parse::<i64>() {
                    Ok(number) => {
                        let entry: &mut Entry = lock.get_mut(&key).unwrap();
                        let new_value = number + numeric_value;
                        entry.update_value(Value::String(new_value.to_string()))?;
                        Ok(number + numeric_value)
                    }
                    Err(_) => Err("ERR value is not an integer or out of range"),
                },
                Value::Vec(_) => {
                    Err("WRONGTYPE Operation against a key holding the wrong kind of value")
                }
                Value::HashSet(_) => {
                    Err("WRONGTYPE Operation against a key holding the wrong kind of value")
                }
            },
            None => {
                self.do_set(&mut lock, &key, Value::String(numeric_value.to_string()))?;
                Ok(numeric_value)
            }
        }
    }

    /// Decrements the number stored at key by increment.
    /// If the key does not exist, it is set to 0 before performing the operation.
    /// An error is returned if the key contains a value of the wrong type or
    /// contains a string that can not be represented as integer.
    /// # Arguments
    ///
    /// * `key` - A string that holds the name of the key to decrement the asociated value.
    /// * `numeric_value` - A i64 number that holds the number to subtract to the old value.
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use proyecto_taller_1::storage::data_storage::DataStorage;
    /// let data = DataStorage::new();
    /// data.decrement_value("key".to_string(), 5);
    /// ```
    ///
    pub fn decrement_value(&self, key: String, numeric_value: i64) -> Result<i64, &'static str> {
        let mut lock = self.data.write().ok().ok_or("Failed to lock database")?;
        let value = lock.get(&key);
        match value {
            Some(val) => match val.value()? {
                Value::String(s) => match s.parse::<i64>() {
                    Ok(number) => {
                        let entry: &mut Entry = lock.get_mut(&key).unwrap();
                        let new_value = number - numeric_value;
                        entry.update_value(Value::String(new_value.to_string()))?;
                        Ok(number - numeric_value)
                    }
                    Err(_j) => Err("Cant decrement a value to a not integer value"),
                },
                Value::Vec(_i) => Err("Cant decrement a value to a vector"),
                Value::HashSet(_j) => Err("Cant decrement a value to a set"),
            },
            None => {
                let negative_value = 0 - numeric_value;
                self.do_set(&mut lock, &key, Value::String(negative_value.to_string()))?;
                Ok(0 - numeric_value)
            }
        }
    }

    /// Push a vector of values to the specified list by appending them to the left of the list.
    /// # Arguments
    ///
    /// * `key` - A string that holds the name of the key to push the new values.
    /// * `vec_values` - A Vector of Strings that holds the new values to push.
    ///
    /// Basic usage:
    ///
    /// ```
    /// use proyecto_taller_1::storage::data_storage::DataStorage;
    /// let data = DataStorage::new();
    /// data.lpushx("key".to_string(), ["hola".to_string()].to_vec());
    /// ```
    ///
    pub fn lpushx(&self, key: String, vec_values: Vec<String>) -> Result<usize, &'static str> {
        self.pushx(key, vec_values, |list, element| list.insert(0, element))
    }

    /// Push a vector of values to the specified list by appending them to the left of the list or creating it.
    /// # Arguments
    ///
    /// * `key` - A string that holds the name of the key to push the new values.
    /// * `vec_values` - A Vector of Strings that holds the new values to push.
    ///
    /// Basic usage:
    ///
    /// ```
    /// use proyecto_taller_1::storage::data_storage::DataStorage;
    /// let data = DataStorage::new();
    /// data.lpush("key".to_string(), ["hola".to_string()].to_vec());
    /// ```
    ///
    pub fn lpush(&self, key: String, vec_values: Vec<String>) -> Result<usize, &'static str> {
        self.push(key, vec_values, |list, element| list.insert(0, element))
    }

    /// Push a vector of values to the specified list by appending them to the right of the list.
    /// # Arguments
    ///
    /// * `key` - A string that holds the name of the key to push the new values.
    /// * `vec_values` - A Vector of Strings that holds the new values to push.
    ///
    /// Basic usage:
    ///
    /// ```
    /// use proyecto_taller_1::storage::data_storage::DataStorage;
    /// let data = DataStorage::new();
    /// data.rpushx("key".to_string(), ["hola".to_string()].to_vec());
    /// ```
    ///
    pub fn rpushx(&self, key: String, vec_values: Vec<String>) -> Result<usize, &'static str> {
        self.pushx(key, vec_values, |list, element| list.push(element))
    }

    /// Push a vector of values to the specified list by appending them to the right of the list.
    /// # Arguments
    ///
    /// * `key` - A string that holds the name of the key to push the new values.
    /// * `vec_values` - A Vector of Strings that holds the new values to push.
    ///
    /// Basic usage:
    ///
    /// ```
    /// use proyecto_taller_1::storage::data_storage::DataStorage;
    /// let data = DataStorage::new();
    /// data.rpush("key".to_string(), ["hola".to_string()].to_vec());
    /// ```
    ///
    pub fn rpush(&self, key: String, vec_values: Vec<String>) -> Result<usize, &'static str> {
        self.push(key, vec_values, |list, element| list.push(element))
    }

    /// Pop count values from the given list.
    /// # Arguments
    ///
    /// * `key` - A string that holds the name of the key to push the new values.
    /// * `count` - A usize number containing the number of items to remove.
    ///
    /// Basic usage:
    ///
    /// ```
    /// use proyecto_taller_1::storage::data_storage::DataStorage;
    /// let data = DataStorage::new();
    /// data.rpop("key".to_string(), 2);
    /// ```
    ///
    pub fn rpop(&self, key: String, count: usize) -> Result<Vec<String>, &'static str> {
        let mut lock = self.data.write().ok().ok_or("Failed to lock database")?;
        let mut result = Vec::new();
        let _ = self.do_apply_vec(key, &mut lock, |list| {
            for _ in 0..count {
                match list.pop() {
                    Some(v) => result.push(v),
                    None => break,
                }
            }
        })?;
        Ok(result)
    }

    /// Push a vector of values to the specified list or create a new if it does not exist.
    fn push(
        &self,
        key: String,
        vec_values: Vec<String>,
        apply: fn(&mut Vec<String>, String) -> (),
    ) -> Result<usize, &'static str> {
        let mut lock = self.data.write().ok().ok_or("Failed to lock database")?;
        match self.do_pushx(key.clone(), vec_values.clone(), &mut lock, apply) {
            Ok(l) => {
                return if l == 0 {
                    self.do_set(&mut lock, &key, Value::Vec(Vec::new()))?;
                    self.do_pushx(key, vec_values, &mut lock, apply)
                } else {
                    Ok(l)
                }
            }
            Err(e) => Err(e),
        }
    }

    /// Push to the list and do nothing if it doesnt exist.
    fn pushx(
        &self,
        key: String,
        vec_values: Vec<String>,
        apply: fn(&mut Vec<String>, String) -> (),
    ) -> Result<usize, &'static str> {
        let mut lock = self.data.write().ok().ok_or("Failed to lock database")?;
        self.do_pushx(key, vec_values, &mut lock, apply)
    }

    /// Push a vector of values into the specified list adding them with the provided function.
    fn do_pushx(
        &self,
        key: String,
        vec_values: Vec<String>,
        lock: &mut RwLockWriteGuard<HashMap<String, Entry>>,
        apply: fn(&mut Vec<String>, String) -> (),
    ) -> Result<usize, &'static str> {
        self.do_apply_vec(key, lock, |vec| {
            for val in &vec_values {
                apply(vec, val.clone());
            }
        })
    }

    /// Sets the list element at index to element.
    /// # Arguments
    ///
    /// * `key` - A string that holds the name of the key to get.
    /// * `index` - A i64 number containing the index position to set de value.
    /// * `value` - A string that holds the value to set.
    ///
    /// Basic usage:
    ///
    /// ```
    /// use proyecto_taller_1::storage::data_storage::DataStorage;
    /// use proyecto_taller_1::storage::data_storage::Value::String;
    /// use proyecto_taller_1::storage::data_storage::Value;
    /// let data = DataStorage::new();
    /// data.set(&"key", Value::Vec(["hola".to_string()].to_vec()));
    /// data.lset("key".to_string(), 0, "mundo".to_string());
    ///
    pub fn lset(&self, key: String, index: i64, value: String) -> Result<(), &'static str> {
        let mut lock = self.data.write().ok().ok_or("Failed to lock database")?;
        let res_entry = self.get_entry(&key, &mut lock);

        match res_entry {
            Ok(opt_entry) => match opt_entry {
                Some(entry) => match entry.value().unwrap() {
                    Value::String(_) => Err("Not list value for that key"),
                    Value::Vec(mut i) => {
                        let index = if index < 0 {
                            (i.len() as i64) + index
                        } else {
                            index
                        };
                        let res = if (index as usize) < i.len() {
                            Ok(index as usize)
                        } else {
                            Err("Index not correct in lset")
                        };
                        match res {
                            Ok(number) => {
                                i[number] = value;
                                entry.update_value(Value::Vec(i))?;
                                Ok(())
                            }
                            Err(s) => Err(s),
                        }
                    }
                    Value::HashSet(_) => Err("Not list value for that key"),
                },
                None => Err("No such key"),
            },
            Err(_) => Err("No such key"),
        }
    }

    /// Removes the first count occurrences of elements equal to element from the list stored at key.
    /// The count argument influences the operation in the following ways:
    /// count > 0: Remove elements equal to element moving from head to tail.
    /// count < 0: Remove elements equal to element moving from tail to head.
    /// count = 0: Remove all elements equal to element.
    /// # Arguments
    ///
    /// * `key` - A string that holds the name of the key to get.
    /// * `index` - A i64 number containing the count argument.
    /// * `value` - A string that holds the value to remove.
    ///
    /// Basic usage:
    ///
    /// ```
    /// use proyecto_taller_1::storage::data_storage::DataStorage;
    /// use proyecto_taller_1::storage::data_storage::Value::String;
    /// use proyecto_taller_1::storage::data_storage::Value;
    /// let data = DataStorage::new();
    /// data.set(&"key", Value::Vec(["hola".to_string(), "hola".to_string()].to_vec()));
    /// data.lrem("key".to_string(), 0, "hola".to_string());
    ///
    pub fn lrem(&self, key: String, index: i64, value: String) -> Result<i64, &'static str> {
        let mut lock = self.data.write().ok().ok_or("Failed to lock database")?;
        let res_entry = self.get_entry(&key, &mut lock);

        match res_entry {
            Ok(opt_entry) => match opt_entry {
                Some(entry) => match entry.value().unwrap() {
                    Value::String(_) => Err("Not list value for that key"),
                    Value::Vec(mut vector) => {
                        let result: i64;
                        match index {
                            index if index < 0 => {
                                let (final_index, new_vector) =
                                    delete_last_values(&mut vector, index.abs(), value);
                                if final_index == 0 {
                                    result = index.abs();
                                } else {
                                    result = final_index;
                                }
                                entry.update_value(Value::Vec(new_vector))?;
                                Ok(result)
                            }
                            index if index == 0 => {
                                let (final_index, new_vector) =
                                    delete_all_values(&mut vector, value);
                                if final_index == 0 {
                                    result = index;
                                } else {
                                    result = final_index;
                                }
                                entry.update_value(Value::Vec(new_vector))?;
                                Ok(result)
                            }
                            _ => {
                                let (final_index, new_vector) =
                                    delete_first_values(&mut vector, index, value);
                                if final_index == 0 {
                                    result = index;
                                } else {
                                    result = final_index;
                                }
                                entry.update_value(Value::Vec(new_vector))?;
                                Ok(result)
                            }
                        }
                    }
                    Value::HashSet(_) => Err("Not list value for that key"),
                },
                None => Ok(0),
            },
            Err(_) => Ok(0),
        }
    }

    /// Returns if the input is a member of the set stored at key.
    /// # Arguments
    ///
    /// * `key` - A string that holds the name of the key to get.
    /// * `input_val` - A String that holds the member to check.
    ///
    /// Basic usage:
    ///
    /// ```
    /// use proyecto_taller_1::storage::data_storage::DataStorage;
    /// use proyecto_taller_1::storage::data_storage::Value::String;
    /// use proyecto_taller_1::storage::data_storage::Value;
    /// use std::collections::HashSet;
    /// let data = DataStorage::new();
    /// let mut set = HashSet::new();
    /// set.insert("hola".to_string());
    /// data.set(&"key", Value::HashSet(set));
    /// data.sismember("key".to_string(), "hola".to_string());
    ///
    pub fn sismember(&self, key: String, input_val: String) -> Result<i64, &'static str> {
        let value = self.get(&key);
        match value {
            Some(val) => match val {
                Value::String(_) => Err("Not set value to that key"),
                Value::Vec(_) => Err("Not set value to that key"),
                Value::HashSet(set) => {
                    if set.contains(&input_val) {
                        Ok(1)
                    } else {
                        Ok(0)
                    }
                }
            },
            None => Ok(0),
        }
    }

    /// Remove the specified members from the set stored at key.
    /// Specified members that are not a member of this set are ignored.
    /// If key does not exist, it is treated as an empty set and this command returns 0.
    /// An error is returned when the value stored at key is not a set.
    /// # Arguments
    ///
    /// * `key` - A string that holds the name of the key to get.
    /// * `values` - A String that holds the members to remove.
    ///
    /// Basic usage:
    ///
    /// ```
    /// use proyecto_taller_1::storage::data_storage::DataStorage;
    /// use proyecto_taller_1::storage::data_storage::Value::String;
    /// use proyecto_taller_1::storage::data_storage::Value;
    /// use std::collections::HashSet;
    /// let data = DataStorage::new();
    /// let mut set = HashSet::new();
    /// set.insert("hola".to_string());
    /// data.set(&"key", Value::HashSet(set));
    /// data.srem("key".to_string(), vec!["hola".to_string()]);
    ///
    pub fn srem(&self, key: String, values: Vec<String>) -> Result<i64, &'static str> {
        let mut lock = self.data.write().ok().ok_or("Failed to lock database")?;
        let res_entry = self.get_entry(&key, &mut lock);

        match res_entry {
            Ok(opt_entry) => match opt_entry {
                Some(entry) => match entry.value().unwrap() {
                    Value::String(_) => Err("Not list value for that key"),
                    Value::Vec(_) => Err("Not list value for that key"),
                    Value::HashSet(mut set) => {
                        let mut count = 0;
                        for value in values {
                            set.remove(&value);
                            count += 1
                        }
                        entry.update_value(Value::HashSet(set))?;
                        Ok(count)
                    }
                },
                None => Ok(0),
            },
            Err(_) => Ok(0),
        }
    }

    /// Add the specified members to the set stored at key.
    /// Specified members that are already a member of this set are ignored.
    /// If key does not exist, a new set is created before adding the specified members.
    /// An error is returned when the value stored at key is not a set.
    /// # Arguments
    ///
    /// * `key` - A string that holds the name of the key to get.
    /// * `values` - A String that holds the members to add.
    ///
    /// Basic usage:
    ///
    /// ```
    /// use proyecto_taller_1::storage::data_storage::DataStorage;
    /// use proyecto_taller_1::storage::data_storage::Value::String;
    /// use proyecto_taller_1::storage::data_storage::Value;
    /// use std::collections::HashSet;
    /// let data = DataStorage::new();
    /// let mut set = HashSet::new();
    /// set.insert("hola".to_string());
    /// data.set(&"key", Value::HashSet(set));
    /// data.sadd("key".to_string(), vec!["mundo".to_string()]);
    ///
    pub fn sadd(&self, key: String, values: Vec<String>) -> Result<i64, &'static str> {
        let mut lock = self.data.write().ok().ok_or("Failed to lock database")?;
        let res_entry = self.get_entry(&key, &mut lock);

        match res_entry {
            Ok(opt_entry) => match opt_entry {
                Some(entry) => match entry.value().unwrap() {
                    Value::String(_) => {
                        Err("WRONGTYPE Operation against a key holding the wrong kind of value")
                    }
                    Value::Vec(_) => {
                        Err("WRONGTYPE Operation against a key holding the wrong kind of value")
                    }
                    Value::HashSet(mut set) => {
                        let mut count = 0;
                        for value in values {
                            if set.insert(value) {
                                count += 1;
                            }
                        }

                        entry.update_value(Value::HashSet(set))?;
                        Ok(count)
                    }
                },
                None => Ok(0),
            },
            Err(_) => {
                let mut new_set = HashSet::new();
                let mut count = 0;
                for value in values {
                    if new_set.insert(value) {
                        count += 1;
                    }
                }
                self.do_set(&mut lock, &key, Value::HashSet(new_set))?;
                Ok(count)
            }
        }
    }

    /// Returns all the members of the set value stored at key.
    /// # Arguments
    ///
    /// * `key` - A string that holds the name of the key to get.
    ///
    /// Basic usage:
    ///
    /// ```
    /// use proyecto_taller_1::storage::data_storage::DataStorage;
    /// use proyecto_taller_1::storage::data_storage::Value::String;
    /// use proyecto_taller_1::storage::data_storage::Value;
    /// use std::collections::HashSet;
    /// let data = DataStorage::new();
    /// let mut set = HashSet::new();
    /// set.insert("hola".to_string());
    /// data.set(&"key", Value::HashSet(set));
    /// let members = data.smember("key".to_string());
    ///
    pub fn smember(&self, key: String) -> Result<Vec<String>, &'static str> {
        let value = self.get(&key);
        match value {
            Some(val) => match val {
                Value::String(_) => Err("Not set value to that key"),
                Value::Vec(_) => Err("Not set value to that key"),
                Value::HashSet(set) => {
                    let vec = set.into_iter().collect();
                    Ok(vec)
                }
            },
            None => Ok([].to_vec()),
        }
    }

    pub fn lrange(
        &self,
        key: String,
        first_index: i64,
        second_index: i64,
    ) -> Result<Option<Vec<String>>, &'static str> {
        let value = self.get(&key);

        match value {
            Some(val) => match val {
                Value::String(_) => Err("Not list value to that key"),
                Value::Vec(vector) => {
                    let mut result: Option<Vec<String>> = None;
                    if first_index >= 0 && second_index >= 0 {
                        result = get_vector_positive_index(
                            vector,
                            first_index as usize,
                            second_index as usize,
                        )
                    } else if first_index < 0 && second_index > 0 {
                        result =
                            get_vector_start_negative(vector, first_index, second_index as usize)
                    } else if first_index > 0 && second_index < 0 {
                        result =
                            get_vector_stop_negative(vector, first_index as usize, second_index)
                    } else if first_index < 0 && second_index < 0 {
                        result = get_vector_negative_index(vector, first_index, second_index)
                    };
                    Ok(result)
                }
                Value::HashSet(_) => Err("Not list value to that key"),
            },
            None => Err("Not value to that key"),
        }
    }
}

fn get_vector_negative_index(
    vector: Vec<String>,
    mut start: i64,
    mut stop: i64,
) -> Option<Vec<String>> {
    if start > vector.len() as i64 {
        start = -(vector.len() as i64);
    };
    if stop > vector.len() as i64 {
        stop = -(vector.len() as i64);
    };

    let new_start = (vector.len() as i64) + start;
    let new_stop = (vector.len() as i64) + stop;
    if new_start > new_stop {
        get_vector_positive_index(vector, new_stop as usize, new_start as usize)
    } else {
        get_vector_positive_index(vector, new_start as usize, new_stop as usize)
    }
}

fn get_vector_stop_negative(
    vector: Vec<String>,
    start: usize,
    mut stop: i64,
) -> Option<Vec<String>> {
    if stop > vector.len() as i64 {
        stop = -(vector.len() as i64);
    };

    let new_stop = (vector.len() as i64) + stop;
    if start as i64 > new_stop {
        get_vector_positive_index(vector, new_stop as usize, start)
    } else {
        get_vector_positive_index(vector, start, new_stop as usize)
    }
}

fn get_vector_start_negative(
    vector: Vec<String>,
    mut start: i64,
    stop: usize,
) -> Option<Vec<String>> {
    if start > vector.len() as i64 {
        start = -(vector.len() as i64);
    };

    let new_start = (vector.len() as i64) + start;
    get_vector_positive_index(vector, new_start as usize, stop)
}

fn get_vector_positive_index(
    vector: Vec<String>,
    start: usize,
    mut stop: usize,
) -> Option<Vec<String>> {
    if start > vector.len() {
        return None;
    };
    if stop >= vector.len() {
        stop = vector.len() - 1;
    };

    let mut result: Vec<String> = vec![];
    for i in vector.iter().take(stop + 1).skip(start) {
        result.push(i.to_string());
    }
    Some(result)
}

fn delete_last_values(
    vector: &mut Vec<String>,
    mut index: i64,
    value: String,
) -> (i64, Vec<String>) {
    let mut new_vector: Vec<String> = vec![];
    for val in vector.iter().rev() {
        if (*val == value) && (index != 0) {
            index -= 1;
        } else {
            new_vector.push(val.to_string());
        }
    }
    (index, new_vector.into_iter().rev().collect())
}

fn delete_first_values(
    vector: &mut Vec<String>,
    mut index: i64,
    value: String,
) -> (i64, Vec<String>) {
    let mut new_vector: Vec<String> = vec![];
    for val in vector.iter() {
        if *val == value && (index != 0) {
            index -= 1;
        } else {
            new_vector.push(val.to_string());
        }
    }
    (index, new_vector)
}

fn delete_all_values(vector: &mut Vec<String>, value: String) -> (i64, Vec<String>) {
    let mut index = 0;
    let mut new_vector: Vec<String> = vec![];
    for val in vector.iter() {
        if *val == value {
            index += 1;
        } else {
            new_vector.push(val.to_string());
        }
    }
    (index, new_vector)
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
