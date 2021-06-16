use crate::storage::data_storage::Value;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

///Structure that contains all the information to store as a value in the database
#[derive(Clone)]
pub struct Entry {
    last_access: Duration,
    key_expiration: Option<Duration>,
    value: Value,
}

#[allow(dead_code)]
impl Entry {
    ///Create a new Entry structure
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
    pub fn set_key_expiration(&mut self, new_expiration: Option<Duration>) {
        self.key_expiration = new_expiration;
    }
}

fn key_is_expired(expiration: Duration) -> bool {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    expiration < now
}
