use redis::Client;

mod common;

#[test]
fn test_subscribe() {
    let (_server, port) = common::setup_server();
    let client1 = common::setup_client(port);

    let mut conn1 = client1.get_connection().unwrap();
    let mut pubsub1 = conn1.as_pubsub();

    pubsub1.subscribe("CHANNEL1").unwrap();
    pubsub1.unsubscribe("CHANNEL1").unwrap();
}

#[test]
fn test_subscribe_and_publish() {
    let (_server, port) = common::setup_server();
    let client1 = common::setup_client(port);
    let client2 = common::setup_client(port);

    let mut conn1 = client1.get_connection().unwrap();
    let mut pubsub1 = conn1.as_pubsub();

    pubsub1.subscribe("CHANNEL1").unwrap();

    let mut count1: u32 = common::query(
        &client2,
        "PUBLISH",
        &["CHANNEL1", "hola este es el canal 1"],
    );
    let msg1 = pubsub1.get_message();

    assert_eq!(count1, 1);
    assert!(msg1.is_ok());
    assert_eq!(
        msg1.unwrap().get_payload::<String>().unwrap(),
        "hola este es el canal 1"
    );

    let count2: u32 = common::query(
        &client2,
        "PUBLISH",
        &["CHANNEL2", "hola este es el canal 2"],
    );

    assert_eq!(count2, 0);
    pubsub1.unsubscribe("CHANNEL1").unwrap();
    count1 = common::query(
        &client2,
        "PUBLISH",
        &["CHANNEL1", "hola este es el canal 1"],
    );
    assert_eq!(count1, 0);
}

#[test]
fn test_multiple_clients() {
    let (_server, port) = common::setup_server();
    let clients: Vec<Client> = vec![0; 3]
        .iter()
        .map(|_x| common::setup_client(port))
        .collect();

    let mut conn1 = clients[0].get_connection().unwrap();
    let mut conn2 = clients[1].get_connection().unwrap();
    let mut pubsub1 = conn1.as_pubsub();
    let mut pubsub2 = conn2.as_pubsub();

    pubsub1.subscribe("CHANNEL1").unwrap();
    pubsub2.subscribe("CHANNEL2").unwrap();

    let count1: u32 = common::query(
        &clients[2],
        "PUBLISH",
        &["CHANNEL1", "hola este es el canal 1"],
    );
    let msg1 = pubsub1.get_message();

    assert_eq!(count1, 1);
    assert!(msg1.is_ok());
    assert_eq!(
        msg1.unwrap().get_payload::<String>().unwrap(),
        "hola este es el canal 1"
    );

    let count2: u32 = common::query(
        &clients[2],
        "PUBLISH",
        &["CHANNEL2", "hola este es el canal 2"],
    );
    let msg2 = pubsub2.get_message();

    assert_eq!(count2, 1);
    assert!(msg2.is_ok());
    assert_eq!(
        msg2.unwrap().get_payload::<String>().unwrap(),
        "hola este es el canal 2"
    );

    pubsub1.unsubscribe("CHANNEL1").unwrap();
    pubsub2.unsubscribe("CHANNEL2").unwrap();
}

#[test]
fn test_multiple_unsubscribe() {
    let (_server, port) = common::setup_server();
    let client1 = common::setup_client(port);

    let mut conn1 = client1.get_connection().unwrap();
    let mut pubsub1 = conn1.as_pubsub();

    pubsub1.subscribe("CHANNEL1").unwrap();
    pubsub1.subscribe("CHANNEL2").unwrap();
    pubsub1.subscribe("CHANNEL3").unwrap();
}

#[test]
fn test_numsub() {
    let (_server, port) = common::setup_server();
    let client1 = common::setup_client(port);
    let client2 = common::setup_client(port);

    let mut conn1 = client1.get_connection().unwrap();
    let mut pubsub1 = conn1.as_pubsub();

    pubsub1.subscribe("CHANNEL1").unwrap();
    pubsub1.subscribe("CHANNEL2").unwrap();
    pubsub1.subscribe("CHANNEL3").unwrap();
    pubsub1.unsubscribe("CHANNEL2").unwrap();

    let r : Vec<u32> = common::query_string(&client2,"PUBSUB NUMSUB CHANNEL1 CHANNEL2 CHANNEL3");
    assert_eq!(r, vec![1, 0, 1]);
}

#[test]
fn test_pubsub_channels_no_pattern() {
    let (_server, port) = common::setup_server();
    let client1 = common::setup_client(port);
    let client2 = common::setup_client(port);

    let mut conn1 = client1.get_connection().unwrap();
    let mut pubsub1 = conn1.as_pubsub();

    pubsub1.subscribe("CHANNEL1").unwrap();
    pubsub1.subscribe("CHANNEL2").unwrap();
    pubsub1.subscribe("CHANNEL3").unwrap();
    pubsub1.unsubscribe("CHANNEL2").unwrap();

    let mut r : Vec<String> = common::query_string(&client2,"PUBSUB CHANNELS");
    r.sort();
    assert_eq!(r, vec!["CHANNEL1", "CHANNEL3"]);
}

#[test]
fn test_pubsub_channels_pattern() {
    let (_server, port) = common::setup_server();
    let client1 = common::setup_client(port);
    let client2 = common::setup_client(port);

    let mut conn1 = client1.get_connection().unwrap();
    let mut pubsub1 = conn1.as_pubsub();

    pubsub1.subscribe("AGE").unwrap();
    pubsub1.subscribe("ATE").unwrap();
    pubsub1.subscribe("HOLA").unwrap();

    let mut r : Vec<String> = common::query_string(&client2,"PUBSUB CHANNELS A?E");
    r.sort();
    assert_eq!(r, vec!["AGE", "ATE"]);
}

