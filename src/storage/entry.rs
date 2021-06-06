use crate::storage::data_storage::Value;
use std::time::Duration;

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

    ///Returns the last access to the key.
    ///This is stored in duration since 1970
    pub fn last_access(&self) -> Duration {
        self.last_access
    }

    ///Returns the expiration of the key if it exists
    ///or None otherwise
    pub fn key_expiration(&self) -> Option<Duration> {
        self.key_expiration
    }

    ///Returns the value store
    pub fn value(&self) -> Value {
        self.value.clone()
    }

    ///Modify the last access to the key
    pub fn set_last_access(&mut self, new_access: Duration) {
        self.last_access = new_access;
    }

    ///Modify the expiration of the key
    pub fn set_key_expiration(&mut self, new_expiration: Option<Duration>) {
        self.key_expiration = new_expiration;
    }
}
