use redis::Commands;

mod common;

#[test]
fn test_set() {
    let (mut server, mut con) = common::setup();
    let _ : () = con.set("my_key", 42).unwrap();
    let clone_val : i32 = con.get("my_key").unwrap();
    assert_eq!(clone_val, 42);
    server.shutdown();
}