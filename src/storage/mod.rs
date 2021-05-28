use crate::storage::data_storage::Value;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
use std::time::Duration;

//MODULOS
pub mod data_storage;
mod file_reader;
pub mod parser;

//TIPOS
type SafeDataStorage = Arc<RwLock<HashMap<String, (Option<Duration>, Value)>>>;
