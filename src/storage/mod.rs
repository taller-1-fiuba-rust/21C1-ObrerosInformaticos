use crate::storage::entry::Entry;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

//MODULOS
pub mod data_storage;
pub mod entry;
mod file_reader;
pub mod parser;

//TIPOS
type SafeDataStorage = Arc<RwLock<HashMap<String, Entry>>>;
