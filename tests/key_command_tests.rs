use redis::Commands;

mod common;

#[test]
fn test_set() {
    let (_, mut client) = common::setup();
    let _ : () = common::query(&client, "set", &["my_key", "42"]);
    let val : i32 = common::query(&client, "get", &["my_key"]);
    assert_eq!(val, 42);
}

#[test]
fn test_copy() {
    let (_, mut client) = common::setup();
    let _ : () = common::query(&client, "set", &["my_key", "Hola"]);
    let _ : () = common::query(&client, "copy", &["my_key", "clone"]);
    let val : String = common::query(&client, "get", &["clone"]);
    assert_eq!(val, "Hola");
}