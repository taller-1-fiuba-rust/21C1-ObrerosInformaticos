mod common;

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
fn test_copy() {
    let (_server, client) = common::setup();
    let _: () = common::query_string(&client, "SET my_key hola");
    let result: i32 = common::query_string(&client, "COPY my_key clone");
    let val: String = common::query_string(&client, "GET clone");
    assert_eq!(result, 1);
    assert_eq!(val, "hola");
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
