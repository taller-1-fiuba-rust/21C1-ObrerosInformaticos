mod common;

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
