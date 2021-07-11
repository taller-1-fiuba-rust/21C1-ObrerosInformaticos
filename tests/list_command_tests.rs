mod common;

#[test]
fn test_lrem() {
    let (_server, client) = common::setup();
    let _: () = common::query_string(&client, "LPUSH my_key 1");
    let _: () = common::query_string(&client, "LPUSH my_key 1");
    let _: () = common::query_string(&client, "LPUSH my_key 2");
    let _: () = common::query_string(&client, "LPUSH my_key 1");
    let val: i32 = common::query_string(&client, "LREM my_key -2 1");
    assert_eq!(val, 2);
    let result: i64 = common::query_string(&client, "LLEN my_key");
    assert_eq!(result, 2);
    let first: i32 = common::query_string(&client, "LINDEX my_key 0");
    let second: i32 = common::query_string(&client, "LINDEX my_key 1");
    assert_eq!(first, 1);
    assert_eq!(second, 2);
}

#[test]
fn test_rpush() {
    let (_server, client) = common::setup();
    let c1: i32 = common::query_string(&client, "RPUSH my_key 1 2 3");
    let val: i32 = common::query_string(&client, "LINDEX my_key 2");
    assert_eq!(c1, 3);
    assert_eq!(val, 3);
}

#[test]
fn test_lset() {
    let (_server, client) = common::setup();
    let _: () = common::query_string(&client, "RPUSH my_key 1 2 3");
    let c1: String = common::query_string(&client, "LSET my_key 1 hola");
    let val: String = common::query_string(&client, "LINDEX my_key 1");
    assert_eq!(c1, "OK");
    assert_eq!(val, "hola");
}

#[test]
fn test_lindex() {
    let (_server, client) = common::setup();
    let c: i32 = common::query_string(&client, "RPUSH my_key 1 2 3");
    let val1: String = common::query_string(&client, "LINDEX my_key 0");
    let val2: String = common::query_string(&client, "LINDEX my_key 1");
    let val3: String = common::query_string(&client, "LINDEX my_key 2");
    let val4: Option<String> = common::query_string(&client, "LINDEX my_key 3");
    assert_eq!(c, 3);
    assert_eq!(val1, "1");
    assert_eq!(val2, "2");
    assert_eq!(val3, "3");
    assert!(val4.is_none());
}

fn vec_compare(va: &Vec<String>, vb: &Vec<String>) -> bool {
    (va.len() == vb.len()) && va.iter().zip(vb).all(|(a, b)| (a == b))
}

#[test]
fn test_lrange() {
    let (_server, client) = common::setup();
    let (): _ = common::query_string(&client, "RPUSH my_key 1 2 3");
    let val1: Vec<String> = common::query_string(&client, "LRANGE my_key 0 2");
    let val2: Vec<String> = common::query_string(&client, "LRANGE my_key 1 -2");
    let val3: Vec<String> = common::query_string(&client, "LRANGE my_key -2 3");
    assert!(vec_compare(
        &val1,
        &vec!["1".to_string(), "2".to_string(), "3".to_string()]
    ));
    assert!(vec_compare(&val2, &vec!["2".to_string()]));
    assert!(vec_compare(&val3, &vec!["2".to_string(), "3".to_string()]));
}

#[test]
fn test_rpushx_no_list() {
    let (_server, client) = common::setup();
    let c: i32 = common::query_string(&client, "RPUSHX my_key 1 2 3 4 5");
    let val: Option<String> = common::query_string(&client, "LINDEX my_key 0");
    assert_eq!(c, 0);
    assert!(val.is_none());
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

#[test]
fn test_llen_nil() {
    let (_server, client) = common::setup();
    let result: i64 = common::query_string(&client, "LLEN no_key");
    assert_eq!(result, 0);
}

#[test]
fn test_lpop_nil() {
    let (_server, client) = common::setup();
    let result: Option<String> = common::query_string(&client, "LPOP no_such_key");
    assert!(result.is_none());
}

#[test]
fn test_llen_many() {
    let (_server, client) = common::setup();
    let c1: i32 = common::query_string(&client, "RPUSH my_key 1 2 3 4 5");
    let val1: i32 = common::query_string(&client, "LLEN my_key");
    let val2: Vec<i32> = common::query_string(&client, "RPOP my_key 3");
    let val3: i32 = common::query_string(&client, "LLEN my_key");
    assert_eq!(c1, 5);
    assert_eq!(val1, 5);
    assert_eq!(val2, vec![5, 4, 3]);
    assert_eq!(val3, 2);
}

#[test]
fn test_lpop_many() {
    let (_server, client) = common::setup();
    let c1: i32 = common::query_string(&client, "RPUSH my_key 1 2 3 4 5");
    let val1: i32 = common::query_string(&client, "LPOP my_key");
    let val2: Vec<i32> = common::query_string(&client, "LPOP my_key 3");
    assert_eq!(c1, 5);
    assert_eq!(val1, 1);
    assert_eq!(val2, vec![2, 3, 4]);
}

#[test]
fn test_lpush() {
    let (_server, client) = common::setup();
    let c: i32 = common::query_string(&client, "LPUSH my_key 1 2 3");
    let val1: String = common::query_string(&client, "LINDEX my_key 0");
    let val2: String = common::query_string(&client, "LINDEX my_key 1");
    let val3: String = common::query_string(&client, "LINDEX my_key 2");
    let val4: Option<String> = common::query_string(&client, "LINDEX my_key 3");
    assert_eq!(c, 3);
    assert_eq!(val1, "3");
    assert_eq!(val2, "2");
    assert_eq!(val3, "1");
    assert!(val4.is_none());
}
