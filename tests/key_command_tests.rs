mod common;

#[test]
fn test_set() {
    let (_server, client) = common::setup();
    let _ : () = common::query_string(&client, "SET my_key 42");
    let val : i32 = common::query_string(&client, "GET my_key");
    assert_eq!(val, 42);
}

#[test]
fn test_copy() {
    let (_server, client) = common::setup();
    let _ : () = common::query_string(&client, "SET my_key hola");
    let _ : () = common::query_string(&client, "COPY my_key clone");
    let val : String = common::query_string(&client, "GET clone");
    assert_eq!(val, "Hola");
}