use amiquip::{Connection, Exchange, Publish, Result};
use serde::Serialize;
use std::{env, process};
use serde_json;

pub mod input;

#[derive(Serialize)]
struct Data {
    time: usize,
    power: f64,
}

fn main() -> Result<()> {

    // read config from filename provided as optional CLI argument
    let args: Vec<String> = env::args().collect();
    let _config = input::Config::new(&args).unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1)
    });

    // Open connection.
    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;

    // Open a channel - None says let the library choose the channel ID.
    let channel = connection.open_channel(None)?;

    // Get a handle to the direct exchange on our channel.
    let exchange = Exchange::direct(&channel);

    let data = Data {
        time: 123456,
        power: std::f64::consts::E,
    };
    let ser_foo = serde_json::to_string(&data).unwrap();

    // Publish a message to the "hello" queue.
    exchange.publish(Publish::new(ser_foo.as_bytes(), "foo"))?;

    connection.close()
}
