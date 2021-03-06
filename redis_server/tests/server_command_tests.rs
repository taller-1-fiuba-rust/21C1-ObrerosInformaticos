use redis::ConnectionLike;
use std::thread::sleep;
use std::time::Duration;

mod common;

#[test]
/// Integration test to test the correct flow of the PING command
fn test_ping() {
    let (_server, client) = common::setup();
    let result: String = common::query_string(&client, "PING");
    assert_eq!(result, "PONG");
}

#[test]
/// Integration test to test the correct flow of the CONFIG command
fn test_config() {
    let (_server, client) = common::setup();
    let result: Vec<i64> = common::query_string(&client, "CONFIG GET verbose");
    let result1: Vec<String> = common::query_string(&client, "CONFIG GET logfile");
    let result2: Vec<i64> = common::query_string(&client, "CONFIG GET timeout");
    let result3: Vec<String> = common::query_string(&client, "CONFIG GET dbfilename");

    let result5: String = common::query_string(&client, "CONFIG SET verbose 1");

    let result6: Vec<i64> = common::query_string(&client, "CONFIG GET verbose");

    assert_eq!(result[0], 0);
    assert_eq!(result1[0], "logfile.txt");
    assert_eq!(result2[0], 0);
    assert_eq!(result3[0], "dump.rdb");
    assert_eq!(result5, "Ok");
    assert_eq!(result6[0], 1);
}

#[test]
#[should_panic]
/// Integration test to test the timeout of the requests
fn test_timeout() {
    let (_server, client) = common::setup();
    let res: String = common::query_string(&client, "CONFIG SET TIMEOUT 1");
    let mut conn = client.get_connection().unwrap();

    assert_eq!(res, "Ok");
    sleep(Duration::from_secs(2));
    let _: () = redis::cmd("INFO").query(&mut conn).unwrap();
}

#[test]
/// Integration test to test the timeout of the requests
fn test_timeout_unlimited() {
    let (_server, client) = common::setup();
    let res: String = common::query_string(&client, "CONFIG SET TIMEOUT 0");
    let mut conn = client.get_connection().unwrap();

    assert_eq!(res, "Ok");
    sleep(Duration::from_secs(2));
    let _: () = redis::cmd("INFO").query(&mut conn).unwrap();
    assert!(conn.is_open());
}
