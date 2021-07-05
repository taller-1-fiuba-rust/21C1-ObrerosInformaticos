mod common;

#[test]
fn test_copy() {
    let (_server, client) = common::setup();
    let _: () = common::query_string(&client, "SET my_key hola");
    let result: i32 = common::query_string(&client, "COPY my_key clone");
    let val: String = common::query_string(&client, "GET clone");
    assert_eq!(result, 1);
    assert_eq!(val, "hola");
}
