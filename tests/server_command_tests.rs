mod common;

#[test]
fn test_ping() {
    let (_server, client) = common::setup();
    let result: String = common::query_string(&client, "PING");
    assert_eq!(result, "PONG");
}

#[test]
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
