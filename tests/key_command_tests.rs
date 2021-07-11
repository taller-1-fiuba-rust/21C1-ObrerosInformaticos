mod common;

#[test]
fn test_del() {
    let (_server, client) = common::setup();
    let _: () = common::query_string(&client, "SET first_key test_del");
    let _: () = common::query_string(&client, "SET second_key test");
    let result: i32 = common::query_string(&client, "DEL first_key second_key");
    let val_1: Option<String> = common::query_string(&client, "GET first_key");
    let val_2: Option<String> = common::query_string(&client, "GET second_key");
    assert_eq!(result, 2);
    assert!(val_1.is_none());
    assert!(val_2.is_none());
}

#[test]
fn test_exists() {
    let (_server, client) = common::setup();
    let _: () = common::query_string(&client, "SET first_key test");
    let _: () = common::query_string(&client, "SET second_key test");
    let val: i32 = common::query_string(&client, "EXISTS first_key second_key second_key key");
    assert_eq!(val, 3);
}

#[test]
fn test_expire() {
    let (_server, client) = common::setup();
    let _: () = common::query_string(&client, "SET first_key test_1");
    let _: () = common::query_string(&client, "EXPIRE first_key 100");
    let result: i32 = common::query_string(&client, "TTL first_key");
    assert_eq!(result, 100);
}

#[test]
fn test_expireat() {
    let (_server, client) = common::setup();
    let _: () = common::query_string(&client, "SET first_key test");
    let _: () = common::query_string(&client, "EXPIREAT first_key 1293840000");
    let result: i32 = common::query_string(&client, "EXISTS first_key");
    assert_eq!(result, 0);
}

#[test]
fn test_persist() {
    let (_server, client) = common::setup();
    let _: () = common::query_string(&client, "SET key test");
    let _: () = common::query_string(&client, "EXPIRE key 100");
    let result: i32 = common::query_string(&client, "TTL key");
    assert!(result <= 100);
    let _: () = common::query_string(&client, "PERSIST key");
    let result_p: i32 = common::query_string(&client, "TTL key");
    assert_eq!(result_p, -1);
}

#[test]
fn test_keys_all() {
    let (_server, client) = common::setup();
    let _: () = common::query_string(&client, "SET first_key test");
    let result: Vec<String> = common::query_string(&client, "KEYS *");
    assert_eq!(result, ["first_key"]);
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
fn test_rename() {
    let (_server, client) = common::setup();
    let _: () = common::query_string(&client, "SET my_key hola");
    let result: String = common::query_string(&client, "RENAME my_key my_key2");
    let val1: Option<String> = common::query_string(&client, "GET my_key");
    let val2: String = common::query_string(&client, "GET my_key2");

    assert_eq!(result, "OK");
    assert!(val1.is_none());
    assert_eq!(val2, "hola");
}

#[test]
fn test_rename_with_exp() {
    let (_server, client) = common::setup();
    let _: () = common::query_string(&client, "SET my_key hola");
    let _: () = common::query_string(&client, "EXPIRE my_key 100");
    let result: String = common::query_string(&client, "RENAME my_key my_key2");
    let val1: i32 = common::query_string(&client, "TTL my_key2");
    let val2: String = common::query_string(&client, "GET my_key2");

    assert_eq!(result, "OK");
    assert!(val1 > 50);
    assert_eq!(val2, "hola");
}

#[test]
fn test_keys2() {
    let (_server, client) = common::setup();
    let _: () = common::query_string(&client, "MSET age 1 ate 1 ame 1 key 1 fisura 1");
    let mut result: Vec<String> = common::query_string(&client, "KEYS a?e");
    result.sort();
    assert_eq!(result, vec!["age", "ame", "ate"]);
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
fn test_ttl() {
    let (_server, client) = common::setup();
    let _: () = common::query_string(&client, "SET my_key hola");
    let result: i32 = common::query_string(&client, "EXPIRE my_key 10");
    let val: i64 = common::query_string(&client, "TTL my_key");
    assert_eq!(result, 1);
    assert!(val > 6 && val <= 10);
}
