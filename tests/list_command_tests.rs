mod common;

#[test]
fn test_rpush() {
    let (_server, client) = common::setup();
    let c: i32 = common::query_string(&client, "RPUSH my_key 1 2 3");
    let val1: String = common::query_string(&client, "LINDEX my_key 0");
    let val2: String = common::query_string(&client, "LINDEX my_key 1");
    let val3: String = common::query_string(&client, "LINDEX my_key 2");
    let val4: String = common::query_string(&client, "LINDEX my_key 3");
    assert_eq!(c, 3);
    assert_eq!(val1, "1");
    assert_eq!(val2, "2");
    assert_eq!(val3, "3");
    assert_eq!(val4, "nil");
}

#[test]
fn test_rpushx_no_list() {
    let (_server, client) = common::setup();
    let c: i32 = common::query_string(&client, "RPUSHX my_key 1 2 3 4 5");
    let val: String = common::query_string(&client, "LINDEX my_key 0");
    assert_eq!(c, 0);
    assert_eq!(val, "nil");
}

#[test]
fn test_rpushx() {
    let (_server, client) = common::setup();
    let c1: i32 = common::query_string(&client, "RPUSH my_key 1");
    let c2: i32 = common::query_string(&client, "RPUSHX my_key 2 3 4 5");
    let val: i32 = common::query_string(&client, "LINDEX my_key 2");
    assert_eq!(c1, 1);
    assert_eq!(c2, 5);
    assert_eq!(val, 3);
}

#[test]
fn test_rpop_nil() {
    let (_server, client) = common::setup();
    let result: Option<String> = common::query_string(&client, "RPOP no_such_key");
    assert!(result.is_none());
}

#[test]
fn test_rpop_many() {
    let (_server, client) = common::setup();
    let c1: i32 = common::query_string(&client, "RPUSH my_key 1 2 3 4 5");
    let val1: i32 = common::query_string(&client, "RPOP my_key");
    let val2: Vec<i32> = common::query_string(&client, "RPOP my_key 3");
    assert_eq!(c1, 5);
    assert_eq!(val1, 5);
    assert_eq!(val2, vec![4, 3, 2]);
}