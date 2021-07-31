use crate::storage::data_storage::Value;
use crate::storage::entry::Entry;
use crate::storage::file_reader;
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Duration;

static LIST: &str = "|LISTA|";
static SET: &str = "|SET|";
static STRING: &str = "|STRING|";

/// Given a file and a data structure get the information from the file
/// and stores it in the structure, respecting the contained data types.
/// # Arguments
///
/// * `file` - A String slice that holds the name of the file to parse.
/// * `data` - A HashMap<String, Entry> where the data obtained from the file will be saved.
///
/// # Example
///
/// Basic usage:
///
/// ```
/// use redis_server::storage::parser;
/// use redis_server::storage::entry::Entry;
/// use std::collections::HashMap;
/// let mut set: HashMap<String, Entry> = HashMap::new();
/// parser::parse_data(&"data_file.txt", &mut set);
/// ```
///
pub fn parse_data(file: &str, data: &mut HashMap<String, Entry>) -> Result<(), &'static str> {
    match file_reader::read_lines(file) {
        Ok(lines) => {
            for line in lines {
                let vec: Vec<&str> = line.split(|c| c == ';').collect();

                if vec[1].contains(LIST) {
                    let (key, entry) = get_vector_data(vec);
                    data.insert(key, entry);
                } else if vec[1].contains(SET) {
                    let (key, entry) = get_set_data(vec);
                    data.insert(key, entry);
                } else {
                    let (key, entry) = get_string_data(vec);
                    data.insert(key, entry);
                };
            }
            Ok(())
        }
        Err(_i) => Err("Could not parse the file"),
    }
}

/// Given a file and a data structure take the information of the structure
/// and stores it in the file, respecting the predefined storage structure.
/// # Arguments
///
/// * `file` - A String slice that holds the name of the file to save the data.
/// * `data` - A HashMap<String, Entry> from where the data will be taken to store.
///
/// # Example
///
/// Basic usage:
///
/// ```
/// use redis_server::storage::parser;
/// use redis_server::storage::entry::Entry;
/// use std::collections::HashMap;
/// let mut set: HashMap<String, Entry> = HashMap::new();
/// parser::store_data(&"data_file.txt", &mut set);
/// ```
///
pub fn store_data(file: &str, data: &HashMap<String, Entry>) {
    for (key, entry) in &*data {
        match entry.value() {
            Ok(value) => match value {
                Value::String(s) => save_string_data(file, key, entry, s),
                Value::Vec(i) => save_vector_data(file, key, entry, &i),
                Value::HashSet(j) => save_set_data(file, key, entry, &j),
            },
            Err(_) => continue,
        };
    }
}

/// Stores information as a string in the file 'file'.
fn save_string_data(file: &str, key: &str, entry: &Entry, value: String) {
    let save_data: String;
    let last_access_secs = entry.last_access().unwrap().as_secs();

    if entry.key_expiration() != Ok(None) {
        let key_expiration_secs = entry.key_expiration().unwrap().unwrap().as_secs();
        save_data = format!(
            "{};{};{};{};{}",
            key, STRING, last_access_secs, key_expiration_secs, value
        );
    } else {
        save_data = format!("{};{};{};{};{}", key, STRING, last_access_secs, 0, value);
    }

    file_reader::data_to_file(file, save_data);
}

/// Stores information in vector form in the file 'file'.
fn save_vector_data(file: &str, key: &str, entry: &Entry, value: &[String]) {
    let values_joined = value.join(",");
    let last_access_secs = entry.last_access().unwrap().as_secs();
    let save_data: String;

    if entry.key_expiration() != Ok(None) {
        let key_expiration_secs = entry.key_expiration().unwrap().unwrap().as_secs();
        save_data = format!(
            "{};{};{};{};{}",
            key, LIST, last_access_secs, key_expiration_secs, values_joined
        );
    } else {
        save_data = format!(
            "{};{};{};{};{}",
            key, LIST, last_access_secs, 0, values_joined
        );
    }

    file_reader::data_to_file(file, save_data);
}

/// Stores information as a set in the file 'file'.
fn save_set_data(file: &str, key: &str, entry: &Entry, value: &HashSet<String>) {
    let set = value.clone();
    let values_joined = set.into_iter().collect::<Vec<String>>().join(",");
    let last_access_secs = entry.last_access().unwrap().as_secs();
    let save_data: String;

    if entry.key_expiration() != Ok(None) {
        let key_expiration_secs = entry.key_expiration().unwrap().unwrap().as_secs();
        save_data = format!(
            "{};{};{};{};{}",
            key, SET, last_access_secs, key_expiration_secs, values_joined
        );
    } else {
        save_data = format!(
            "{};{};{};{};{}",
            key, SET, last_access_secs, 0, values_joined
        );
    }

    file_reader::data_to_file(file, save_data);
}

/// Get the information in the form of a string from the file 'file'.
fn get_string_data(vec: Vec<&str>) -> (String, Entry) {
    let key = vec[0].to_string();

    let last_access_number = vec[2].parse::<u64>().unwrap();
    let last_access = Duration::from_secs(last_access_number);
    let key_expiration_number = vec[3].parse::<u64>().unwrap();
    let key_expiration: Option<Duration>;
    if key_expiration_number != 0 {
        key_expiration = Some(Duration::from_secs(key_expiration_number));
    } else {
        key_expiration = None;
    }

    let value = vec[4].to_string();

    (
        key,
        Entry::new(last_access, key_expiration, Value::String(value)),
    )
}

/// Get the information in vector form of the file 'file'.
fn get_vector_data(mut vec: Vec<&str>) -> (String, Entry) {
    let mut data: Vec<String> = vec![];
    let key = vec[0].to_string();
    vec.remove(0);
    vec.remove(0);

    let last_access_number = vec[0].parse::<u64>().unwrap();
    let last_access = Duration::from_secs(last_access_number);
    vec.remove(0);

    let key_expiration_number = vec[0].parse::<u64>().unwrap();
    let key_expiration: Option<Duration>;

    if key_expiration_number != 0 {
        key_expiration = Some(Duration::from_secs(key_expiration_number));
    } else {
        key_expiration = None;
    }
    vec.remove(0);

    let values: Vec<&str> = vec[0].split(|c| c == ',').collect();

    for element in values {
        data.push(element.to_string());
    }

    (
        key,
        Entry::new(last_access, key_expiration, Value::Vec(data)),
    )
}

/// Get the information in the form of a set from the file 'file'.
fn get_set_data(mut vec: Vec<&str>) -> (String, Entry) {
    let mut data: HashSet<String> = HashSet::new();
    let key = vec[0].to_string();
    vec.remove(0);
    vec.remove(0);

    let last_access_number = vec[0].parse::<u64>().unwrap();
    let last_access = Duration::from_secs(last_access_number);
    vec.remove(0);

    let key_expiration_number = vec[0].parse::<u64>().unwrap();
    let key_expiration: Option<Duration>;

    if key_expiration_number != 0 {
        key_expiration = Some(Duration::from_secs(key_expiration_number));
    } else {
        key_expiration = None;
    }
    vec.remove(0);

    let values: Vec<&str> = vec[0].split(|c| c == ',').collect();

    for element in values {
        data.insert(element.to_string());
    }

    (
        key,
        Entry::new(last_access, key_expiration, Value::HashSet(data)),
    )
}
