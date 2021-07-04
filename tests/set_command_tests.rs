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
