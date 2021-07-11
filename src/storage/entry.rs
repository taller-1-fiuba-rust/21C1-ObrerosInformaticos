use crate::storage::data_storage::Value;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

///Structure that contains all the information to store as a value in the database
/// # Arguments
///
/// * `last_access` - A Duration slice that holds the last access of the key.
/// * `key_expiration` - A Option<Duration> slice that holds the key expiration or None in case it has not been set.
/// * `value` - A Value that contains the member to store in the entry.
///
#[derive(Clone)]
pub struct Entry {
    last_access: Duration,
    key_expiration: Option<Duration>,
    value: Value,
}

impl Entry {
    /// Create a new Entry structure
    /// # Arguments
    ///
    /// * `last_access` - A Duration slice that holds the last access of the key.
    /// * `key_expiration` - A Option<Duration> slice that holds the key expiration or None in case it has not been set.
    /// * `value` - A Value that contains the member to store in the entry.
    ///
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use proyecto_taller_1::storage::entry::Entry;
    /// use proyecto_taller_1::storage::data_storage::Value::String;
    /// use proyecto_taller_1::storage::data_storage::Value;
    /// use std::time::SystemTime;
    /// use std::time::UNIX_EPOCH;
    /// let value = Value::String("hola".to_string());
    /// let entry = Entry::new(SystemTime::now().duration_since(UNIX_EPOCH).unwrap(), None, value);
    /// ```
    ///
    pub fn new(last_access: Duration, key_expiration: Option<Duration>, value: Value) -> Self {
        Entry {
            last_access,
            key_expiration,
            value,
        }
    }

    ///Returns the last access to the key if
    ///the key is not expired or an error otherwise.
    ///This is stored in duration since 1970
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use proyecto_taller_1::storage::entry::Entry;
    /// use proyecto_taller_1::storage::data_storage::Value::String;
    /// use proyecto_taller_1::storage::data_storage::Value;
    /// use std::time::SystemTime;
    /// use std::time::UNIX_EPOCH;
    /// let value = Value::String("hola".to_string());
    /// let entry = Entry::new(SystemTime::now().duration_since(UNIX_EPOCH).unwrap(), None, value);
    /// let last_acces = entry.last_access();
    /// ```
    ///
    pub fn last_access(&self) -> Result<Duration, &'static str> {
        let key_is_expired = match self.key_expiration {
            Some(exp) => key_is_expired(exp),
            None => false,
        };
        if key_is_expired {
            Err("Key expired")
        } else {
            Ok(self.last_access)
        }
    }

    ///Returns the expiration of the key if
    ///the key is not expired or an error otherwise
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use proyecto_taller_1::storage::entry::Entry;
    /// use proyecto_taller_1::storage::data_storage::Value::String;
    /// use proyecto_taller_1::storage::data_storage::Value;
    /// use std::time::SystemTime;
    /// use std::time::UNIX_EPOCH;
    /// let value = Value::String("hola".to_string());
    /// let entry = Entry::new(SystemTime::now().duration_since(UNIX_EPOCH).unwrap(), None, value);
    /// let key_expiration = entry.key_expiration();
    /// ```
    ///
    pub fn key_expiration(&self) -> Result<Option<Duration>, &'static str> {
        let key_is_expired = match self.key_expiration {
            Some(exp) => key_is_expired(exp),
            None => false,
        };
        if key_is_expired {
            Err("Key expired")
        } else {
            Ok(self.key_expiration)
        }
    }

    ///Returns the value store if the key is not expired or an error otherwise.
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use proyecto_taller_1::storage::entry::Entry;
    /// use proyecto_taller_1::storage::data_storage::Value::String;
    /// use proyecto_taller_1::storage::data_storage::Value;
    /// use std::time::SystemTime;
    /// use std::time::UNIX_EPOCH;
    /// let value = Value::String("hola".to_string());
    /// let entry = Entry::new(SystemTime::now().duration_since(UNIX_EPOCH).unwrap(), None, value);
    /// let value = entry.value();
    /// ```
    ///
    pub fn value(&self) -> Result<Value, &'static str> {
        let key_is_expired = match self.key_expiration {
            Some(exp) => key_is_expired(exp),
            None => false,
        };
        if key_is_expired {
            Err("Key expired")
        } else {
            Ok(self.value.clone())
        }
    }

    ///Update the value if the key is not expired or an error otherwise.
    /// # Arguments
    ///
    /// * `new_value` - A Value that contains the new member to store in the entry.
    ///
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use proyecto_taller_1::storage::entry::Entry;
    /// use proyecto_taller_1::storage::data_storage::Value::String;
    /// use proyecto_taller_1::storage::data_storage::Value;
    /// use std::time::SystemTime;
    /// use std::time::UNIX_EPOCH;
    /// let value = Value::String("hola".to_string());
    /// let mut entry = Entry::new(SystemTime::now().duration_since(UNIX_EPOCH).unwrap(), None, value);
    /// let new_value = Value::String("mundo".to_string());
    /// entry.update_value(new_value);
    /// ```
    ///
    pub fn update_value(&mut self, new_value: Value) -> Result<(), &'static str> {
        let key_is_expired = match self.key_expiration {
            Some(exp) => key_is_expired(exp),
            None => false,
        };
        if key_is_expired {
            Err("Key expired")
        } else {
            self.value = new_value;
            Ok(())
        }
    }

    ///Modify the last access to the key if the key is not expired or an error otherwise.
    /// # Arguments
    ///
    /// * `new_access` - A Duration that contains the new last access to store in the entry.
    ///
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use proyecto_taller_1::storage::entry::Entry;
    /// use proyecto_taller_1::storage::data_storage::Value::String;
    /// use proyecto_taller_1::storage::data_storage::Value;
    /// use std::time::SystemTime;
    /// use std::time::UNIX_EPOCH;
    /// let value = Value::String("hola".to_string());
    /// let mut entry = Entry::new(SystemTime::now().duration_since(UNIX_EPOCH).unwrap(), None, value);
    /// let new_access = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    /// entry.set_last_access(new_access);
    /// ```
    ///
    pub fn set_last_access(&mut self, new_access: Duration) -> Result<(), &'static str> {
        let key_is_expired = match self.key_expiration {
            Some(exp) => key_is_expired(exp),
            None => false,
        };
        if key_is_expired {
            Err("Key expired")
        } else {
            self.last_access = new_access;
            Ok(())
        }
    }

    ///Modify the expiration of the key
    /// # Arguments
    ///
    /// * `new_expiration` - A Option<Duration> that contains the Duration to store in the entry.
    ///
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use proyecto_taller_1::storage::entry::Entry;
    /// use proyecto_taller_1::storage::data_storage::Value::String;
    /// use proyecto_taller_1::storage::data_storage::Value;
    /// use std::time::SystemTime;
    /// use std::time::Duration;
    /// use std::time::UNIX_EPOCH;
    /// let value = Value::String("hola".to_string());
    /// let mut entry = Entry::new(SystemTime::now().duration_since(UNIX_EPOCH).unwrap(), None, value);
    /// let duration = Some(Duration::from_secs(10));
    /// entry.set_key_expiration(duration);
    /// ```
    ///
    pub fn set_key_expiration(&mut self, new_expiration: Option<Duration>) {
        self.key_expiration = new_expiration;
    }
}

fn key_is_expired(expiration: Duration) -> bool {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    expiration < now
}
