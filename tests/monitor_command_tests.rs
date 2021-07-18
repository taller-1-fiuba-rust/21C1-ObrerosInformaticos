use redis::FromRedisValue;

mod common;

#[test]
fn test_monitor() {
    let (_server, port) = common::setup_server();
    let client1 = common::setup_client(port);
    let client2 = common::setup_client(port);

    let mut monitor_conn = client1.get_connection().unwrap();
    let _: () = redis::cmd("MONITOR").query(&mut monitor_conn).unwrap();
    let _: () = common::query_string(&client2, "KEYS *");

    let msg1 = (&mut monitor_conn).recv_response().unwrap();

    assert_eq!(String::from_redis_value(&msg1).unwrap(), "KEYS *");

    let _: () = common::query_string(&client1, "QUIT");
}
