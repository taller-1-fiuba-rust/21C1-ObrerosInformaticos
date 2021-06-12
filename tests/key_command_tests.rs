use redis::Commands;

mod common;

#[test]
fn test_set() {
    let (mut server, mut client) = common::setup();
    let _ : () = redis::cmd("set").arg("my_key").arg(42).query(&mut client.get_connection().unwrap()).unwrap();
    let clone_val : i32 = redis::cmd("get").arg("my_key").query(&mut client.get_connection().unwrap()).unwrap();
    assert_eq!(clone_val, 42);
    server.shutdown();
}