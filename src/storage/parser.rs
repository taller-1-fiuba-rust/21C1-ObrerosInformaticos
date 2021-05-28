use crate::storage::data_storage::Value;
use crate::storage::file_reader;
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Duration;

static LIST: &str = "|LISTA|";
static SET: &str = "|SET|";

pub fn parse_data(file: &str, data: &mut HashMap<String, (Option<Duration>, Value)>) {
    let lines = file_reader::read_lines(file);

    for line in lines {
        let vec: Vec<&str> = line.split(|c| c == ';').collect();

        if vec[1].contains(LIST) {
            let (key, key_expiration, vector) = get_vector_data(vec);
            data.insert(key, (key_expiration, Value::Vec(vector)));
        } else if vec[1].contains(SET) {
            let (key, key_expiration, set) = get_set_data(vec);
            data.insert(key, (key_expiration, Value::HashSet(set)));
        } else {
            let (key, key_expiration, string) = get_string_data(vec);
            data.insert(key, (key_expiration, Value::String(string)));
        };
    }
}

//Para almacenar una clave con expiracion "None" (sin expiracion) se almacena un 0.
pub fn store_data(file: &str, data: &HashMap<String, (Option<Duration>, Value)>) {
    for (key, value) in &*data {
        match &value.1 {
            Value::String(s) => save_string_data(file, key, (value.0, s)),
            Value::Vec(i) => save_vector_data(file, key, (value.0, i)),
            Value::HashSet(j) => save_set_data(file, key, (value.0, j)),
        };
    }
}

fn save_string_data(file: &str, key: &str, value: (Option<Duration>, &str)) {
    let save_data: String;

    if value.0 != None {
        save_data = format!("{};{};{}", key, value.0.unwrap().as_secs(), value.1);
    } else {
        save_data = format!("{};{};{}", key, 0, value.1);
    }

    file_reader::data_to_file(file, save_data);
}

fn save_vector_data(file: &str, key: &str, value: (Option<Duration>, &[String])) {
    let values_joined = (value.1).join(",");
    let save_data: String;

    if value.0 != None {
        save_data = format!(
            "{};{};{};{}",
            key,
            LIST,
            value.0.unwrap().as_secs(),
            values_joined
        );
    } else {
        save_data = format!("{};{};{};{}", key, LIST, 0, values_joined);
    }

    file_reader::data_to_file(file, save_data);
}

fn save_set_data(file: &str, key: &str, value: (Option<Duration>, &HashSet<String>)) {
    let set = value.1.clone();
    let values_joined = set.into_iter().collect::<Vec<String>>().join(",");
    let save_data: String;

    if value.0 != None {
        save_data = format!(
            "{};{};{};{}",
            key,
            SET,
            value.0.unwrap().as_secs(),
            values_joined
        );
    } else {
        save_data = format!("{};{};{};{}", key, SET, 0, values_joined);
    }

    file_reader::data_to_file(file, save_data);
}

fn get_string_data(vec: Vec<&str>) -> (String, Option<Duration>, String) {
    let key = vec[0].to_string();

    let key_expiration_number = vec[1].parse::<u64>().unwrap();
    let key_expiration: Option<Duration>;
    if key_expiration_number != 0 {
        key_expiration = Some(Duration::from_secs(key_expiration_number));
    } else {
        key_expiration = None;
    }

    let value = vec[2].to_string();

    (key, key_expiration, value)
}

fn get_vector_data(mut vec: Vec<&str>) -> (String, Option<Duration>, Vec<String>) {
    let mut data: Vec<String> = vec![];
    let key = vec[0].to_string();
    vec.remove(0);
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

    (key, key_expiration, data)
}

fn get_set_data(mut vec: Vec<&str>) -> (String, Option<Duration>, HashSet<String>) {
    let mut data: HashSet<String> = HashSet::new();
    let key = vec[0].to_string();
    vec.remove(0);
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

    (key, key_expiration, data)
}
