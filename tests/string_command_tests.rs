mod common;

#[test]
fn test_append() {
    let (_server, client) = common::setup();
    let result: i64 = common::query_string(&client, "EXISTS mykey");
    assert_eq!(result, 0);
    let val1: i64 = common::query_string(&client, "APPEND mykey Hello");
    let val2: i64 = common::query_string(&client, "APPEND mykey World");
    let val3: String = common::query_string(&client, "GET mykey");
    assert_eq!(val1, 5);
    assert_eq!(val2, 10);
    assert_eq!(val3, "HelloWorld");
}

#[test]
fn test_decrby() {
    let (_server, client) = common::setup();
    let result: String = common::query_string(&client, "SET key1 10");
    assert_eq!(result, "OK");
    let result: i64 = common::query_string(&client, "DECRBY key1 3");
    assert_eq!(result, 7);
}

#[test]
fn test_getdel() {
    let (_server, client) = common::setup();
    let result: String = common::query_string(&client, "SET key1 Hello");
    assert_eq!(result, "OK");
    let val: String = common::query_string(&client, "GETDEL key1");
    assert_eq!(val, "Hello");
    let result: i64 = common::query_string(&client, "EXISTS key1");
    assert_eq!(result, 0);
}

#[test]
fn test_mget() {
    let (_server, client) = common::setup();
    let result: String = common::query_string(&client, "SET key1 Hello");
    assert_eq!(result, "OK");
    let result: String = common::query_string(&client, "SET key2 World");
    assert_eq!(result, "OK");
    let val: Vec<String> = common::query_string(&client, "MGET key1 key2 key3");
    assert_eq!(val[0], "Hello");
    assert_eq!(val[1], "World");
    assert_eq!(val.len(), 2);
}

#[test]
fn test_strlen() {
    let (_server, client) = common::setup();
    let result: String = common::query_string(&client, "SET mykey Hello");
    assert_eq!(result, "OK");
    let val1: i64 = common::query_string(&client, "STRLEN mykey");
    assert_eq!(val1, 5);
}

#[test]
fn test_incrby_ok() {
    let (_server, client) = common::setup();
    let result: String = common::query_string(&client, "SET my_key 5");
    let val1: i64 = common::query_string(&client, "INCRBY my_key 5");
    let val2: i64 = common::query_string(&client, "GET my_key");
    let val3: i64 = common::query_string(&client, "INCRBY my_key 10");
    let val4: i64 = common::query_string(&client, "GET my_key");
    assert_eq!(result, "OK");
    assert_eq!(val1, 10);
    assert_eq!(val2, 10);
    assert_eq!(val3, 20);
    assert_eq!(val4, 20);
}

#[test]
fn test_set() {
    let (_server, client) = common::setup();
    let result: String = common::query_string(&client, "SET my_key 42");
    let val: i32 = common::query_string(&client, "GET my_key");
    assert_eq!(result, "OK");
    assert_eq!(val, 42);
}

#[test]
fn test_strlen() {
    let (_server, client) = common::setup();
    let result: String = common::query_string(&client, "SET my_key asd");
    let val: i32 = common::query_string(&client, "STRLEN my_key");
    assert_eq!(result, "OK");
    assert_eq!(val, 3);
}

#[test]
fn test_set_get() {
    let (_server, client) = common::setup();
    let _: () = common::query_string(&client, "SET my_key 42");
    let prev: i32 = common::query_string(&client, "SET my_key Hola GET");
    let val: String = common::query_string(&client, "GET my_key");
    assert_eq!(prev, 42);
    assert_eq!(val, "Hola");
}

#[test]
fn test_mset() {
    let (_server, client) = common::setup();
    let result: String = common::query_string(&client, "MSET my_key1 hola1 my_key2 hola2");
    let val1: String = common::query_string(&client, "GET my_key1");
    let val2: String = common::query_string(&client, "GET my_key2");
    assert_eq!(result, "OK");
    assert_eq!(val1, "hola1");
    assert_eq!(val2, "hola2");
}

#[test]
fn test_getset() {
    let (_server, client) = common::setup();
    let result: String = common::query_string(&client, "SET my_key1 hola1");
    let val1: String = common::query_string(&client, "GETSET my_key1 hola2");
    let val2: String = common::query_string(&client, "GETSET my_key2 adios");
    let val3: String = common::query_string(&client, "GET my_key1");
    assert_eq!(result, "OK");
    assert_eq!(val1, "hola1");
    assert_eq!(val2, "nil");
    assert_eq!(val3, "hola2");
}
