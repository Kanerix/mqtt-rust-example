use paho_mqtt as mqtt;
use std::thread;
use std::io::{stdin, stdout, Write};
use std::time;

fn main() {
    let client_options = mqtt::CreateOptionsBuilder::new()
        .client_id("rust_mqtt_client")
        .server_uri("wss://mqtt.nextservices.dk")
        .finalize();
    let client = mqtt::Client::new(client_options).unwrap();

    let connect_options = mqtt::ConnectOptionsBuilder::new()
        .ssl_options(mqtt::SslOptions::default())
        .keep_alive_interval(time::Duration::from_secs(30))
        .finalize();
    client.connect(connect_options).unwrap();

    client.subscribe("mqtt_test_pc", 0).unwrap();

    stdout().write(b"Send message: ").unwrap();
    stdout().flush().unwrap();

    let mut message = String::new();
    stdin().read_line(&mut message).unwrap();

    client
        .publish(mqtt::Message::new("mqtt_test_m5", message, 0))
        .unwrap();
    
    println!("\nAwaiting message...");

    let rx = client.start_consuming();

    let reader = thread::spawn(move || match rx.recv().unwrap() {
        Some(message) => {
            println!("Incoming: \"{}\"", message.payload_str());
        }
        None => {
            println!("No message received");
        }
    });

    reader.join().unwrap();

    client
        .disconnect(mqtt::DisconnectOptions::default())
        .unwrap();
}
