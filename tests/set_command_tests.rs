mod common;

#[test]
fn test_sadd_add_different_values() {
    let (_server, client) = common::setup();
    let c: i32 = common::query_string(&client, "SADD my_key 1 2 3");
    let val2: i64 = common::query_string(&client, "SISMEMBER my_key 1");
    let val3: i64 = common::query_string(&client, "SISMEMBER my_key 2");
    let val4: i64 = common::query_string(&client, "SISMEMBER my_key 3");
    assert_eq!(c, 3);
    assert_eq!(val2, 1);
    assert_eq!(val3, 1);
    assert_eq!(val4, 1);

    let val5: i64 = common::query_string(&client, "SREM my_key 3");
    assert_eq!(val5, 1);
    let val6: i64 = common::query_string(&client, "SISMEMBER my_key 3");
    assert_eq!(val6, 0);
}

#[test]
fn test_sadd_add_different_values_some_old() {
    let (_server, client) = common::setup();
    let first_add: i32 = common::query_string(&client, "SADD my_key 1 2 3");
    let val2: i64 = common::query_string(&client, "SISMEMBER my_key 1");
    let val3: i64 = common::query_string(&client, "SISMEMBER my_key 2");
    let val4: i64 = common::query_string(&client, "SISMEMBER my_key 3");
    assert_eq!(first_add, 3);
    assert_eq!(val2, 1);
    assert_eq!(val3, 1);
    assert_eq!(val4, 1);

    let second_add: i64 = common::query_string(&client, "SADD my_key 3 4 5");
    assert_eq!(second_add, 2);
    let val8: i64 = common::query_string(&client, "SISMEMBER my_key 4");
    assert_eq!(val8, 1);
}

#[test]
fn test_scard() {
    let (_server, client) = common::setup();
    let first_add: i32 = common::query_string(&client, "SADD my_key 1 2 3");
    let val2: i64 = common::query_string(&client, "SISMEMBER my_key 1");
    let val3: i64 = common::query_string(&client, "SISMEMBER my_key 2");
    let val4: i64 = common::query_string(&client, "SISMEMBER my_key 3");
    let val5: i64 = common::query_string(&client, "SCARD my_key");
    assert_eq!(first_add, 3);
    assert_eq!(val2, 1);
    assert_eq!(val3, 1);
    assert_eq!(val4, 1);
    assert_eq!(val5, 3);

    let second_add: i64 = common::query_string(&client, "SADD my_key 3 4 5");
    let val6: i64 = common::query_string(&client, "SCARD my_key");
    assert_eq!(second_add, 2);
    assert_eq!(val6, 5);
    let val7: i64 = common::query_string(&client, "SREM my_key 2 5");
    let val8: i64 = common::query_string(&client, "SISMEMBER my_key 4");
    let val9: i64 = common::query_string(&client, "SCARD my_key");
    assert_eq!(val8, 1);
    assert_eq!(val7, 2);
    assert_eq!(val9, 3);
}

#[test]
fn test_smembers() {
    let (_server, client) = common::setup();
    let _: () = common::query_string(&client, "SADD my_key 1 2 3");
    let mut val: Vec<String> = common::query_string(&client, "SMEMBERS my_key");
    val.sort();
    assert_eq!(val, ["1", "2", "3"]);
}

#[test]
fn sismember() {
    let (_server, client) = common::setup();
    let add: i32 = common::query_string(&client, "SADD my_key 1 2 3");
    let val2: i64 = common::query_string(&client, "SISMEMBER my_key 1");
    let val3: i64 = common::query_string(&client, "SISMEMBER my_key 2");
    let val4: i64 = common::query_string(&client, "SISMEMBER my_key 3");
    assert_eq!(add, 3);
    assert_eq!(val2, 1);
    assert_eq!(val3, 1);
    assert_eq!(val4, 1);
}

#[test]
fn srem() {
    let (_server, client) = common::setup();
    let add: i32 = common::query_string(&client, "SADD my_key 1 2 3");
    let val: i64 = common::query_string(&client, "SREM my_key 2 3");
    let mut result: Vec<String> = common::query_string(&client, "SMEMBERS my_key");
    result.sort();
    assert_eq!(add, 3);
    assert_eq!(val, 2);
    assert_eq!(result, ["1"]);
}