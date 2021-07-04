mod common;

#[test]
fn test_ping() {
    let (_server, client) = common::setup();
    let result: String = common::query_string(&client, "PING");
    assert_eq!(result, "PONG");
}

#[test]
fn test_config() {}
