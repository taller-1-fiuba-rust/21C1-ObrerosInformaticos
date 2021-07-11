mod common;

#[test]
fn test_1() {
    let (_server, client) = common::setup();
    let _: () = common::query_string(&client, "SET first_key test");
    let val2: String = common::query_string(&client, "GET first_key");
    let val3: Vec<i64> = common::query_string(&client, "CONFIG GET verbose");
    let val4: String = common::query_string(&client, "CONFIG SET verbose 1");
    let val5: String = common::query_string(&client, "TYPE first_key");
    let val6: i64 = common::query_string(&client, "TTL first_key");
    let val8: i64 = common::query_string(&client, "LPUSH second_key hola");
    let val9: i64 = common::query_string(&client, "LPUSH second_key test adios");
    let val10: String = common::query_string(&client, "LINDEX second_key 1");
    let val11: Vec<String> = common::query_string(&client, "CONFIG GET dbfilename");
    let val12: i64 = common::query_string(&client, "DEL first_key");
    let val13: Vec<String> = common::query_string(&client, "KEYS *");
    let val14: i64 = common::query_string(&client, "TOUCH first_key second_key");
    let val15: i64 = common::query_string(&client, "LLEN second_key");

    assert_eq!(val2, "test");
    assert_eq!(val3, [0]);
    assert_eq!(val4, "Ok");
    assert_eq!(val5, "string");
    assert_eq!(val6, -1);
    assert_eq!(val8, 1);
    assert_eq!(val9, 3);
    assert_eq!(val10, "test");
    assert_eq!(val11, ["dump.rdb"]);
    assert_eq!(val12, 1);
    assert_eq!(val13, ["second_key"]);
    assert_eq!(val14, 1);
    assert_eq!(val15, 3);
}

#[test]
fn test_2() {
    let (_server, client) = common::setup();
    let _: String = common::query_string(&client, "CONFIG SET verbose 1");

    let val1: i64 = common::query_string(&client, "SADD first_key test asd");
    let val2: i64 = common::query_string(&client, "SCARD first_key");
    let val3: i64 = common::query_string(&client, "EXPIRE first_key 100");
    let val4: i64 = common::query_string(&client, "TTL first_key");
    let val5: String = common::query_string(&client, "RENAME first_key second_key");
    let val8: Vec<String> = common::query_string(&client, "SMEMBERS second_key");
    let val9: i64 = common::query_string(&client, "SADD second_key test adios");
    let val10: i64 = common::query_string(&client, "SREM second_key test");
    let val11: i64 = common::query_string(&client, "RPUSH first_key andy jalife");
    let val12: String = common::query_string(&client, "PING");
    let val13: i64 = common::query_string(&client, "EXISTS first_key first_key third_key");

    assert_eq!(val1, 2);
    assert_eq!(val2, 2);
    assert_eq!(val3, 1);
    assert_eq!(val4, 100);
    assert_eq!(val5, "OK");
    assert!(val8 == ["test", "asd"] || val8 == ["asd", "test"]);
    assert_eq!(val9, 1);
    assert_eq!(val10, 1);
    assert_eq!(val11, 2);
    assert_eq!(val12, "PONG");
    assert_eq!(val13, 2);
}