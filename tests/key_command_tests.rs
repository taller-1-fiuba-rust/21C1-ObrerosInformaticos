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

#[test]
fn test_type() {
    let (_server, client) = common::setup();
    let val1: String = common::query_string(&client, "SET my_key1 value");
    let val2: i64 = common::query_string(&client, "RPUSH my_key2 value");
    let val3: i64 = common::query_string(&client, "SADD my_key3 value");

    let val4: String = common::query_string(&client, "TYPE my_key1");
    let val5: String = common::query_string(&client, "TYPE my_key2");
    let val6: String = common::query_string(&client, "TYPE my_key3");

    assert_eq!(val1, "OK");
    assert_eq!(val2, 1);
    assert_eq!(val3, 1);
    assert_eq!(val4, "string");
    assert_eq!(val5, "vec");
    assert_eq!(val6, "set");
}

#[test]
fn test_sort() {
    let (_server, client) = common::setup();
    let val1: i64 = common::query_string(&client, "RPUSH my_key1 3 2 6");
    let val2: i64 = common::query_string(&client, "SADD my_key2 6 7 3 9");
    assert_eq!(val1, 3);
    assert_eq!(val2, 4);

    let val3: Vec<i64> = common::query_string(&client, "SORT my_key1");
    let val4: Vec<i64> = common::query_string(&client, "SORT my_key2");
    let ok = vec![2, 3, 6];
    for i in 0..2 {
        assert_eq!(ok[i], val3[i]);
    }

    let ok2 = vec![3, 6, 7, 9];
    for i in 0..3 {
        assert_eq!(ok2[i], val4[i]);
    }

    let val5: Vec<i64> = common::query_string(&client, "SORT my_key1 desc");
    let val6: Vec<i64> = common::query_string(&client, "SORT my_key2 desc");
    let ok3 = vec![6, 3, 2];
    for i in 0..2 {
        assert_eq!(ok3[i], val5[i]);
    }

    let ok4 = vec![9, 7, 6, 3];
    for i in 0..3 {
        assert_eq!(ok4[i], val6[i]);
    }
}

#[test]
fn test_touch() {}

#[test]
fn test_ttl() {}
