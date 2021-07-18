mod common;

#[test]
fn test_monitor() {
    let (_server, port) = common::setup_server();
    let client1 = common::setup_client(port);
    let client2 = common::setup_client(port);

    let mut conn1 = client1.get_connection().unwrap();
    let mut monitor = conn1.as_pubsub();

    let _: () = common::query_string(&monitor, "MONITOR");
    let _: () = common::query_string(&client2, "keys *");

    let msg1 = monitor.get_message();

    assert_eq!(
        msg1.unwrap().get_payload::<String>().unwrap(),
        "keys *"
    );
}