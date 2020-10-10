use amiquip::{Connection, Exchange, Publish, Result};
use serde::Serialize;
use serde_json;
use std::{env, process};

pub mod input;

#[derive(Serialize)]
struct Data {
    time: usize,
    power: f64,
}

fn main() -> Result<()> {
    // read config from filename provided as optional CLI argument
    let args: Vec<String> = env::args().collect();
    let config = input::Config::new(&args).unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1)
    });

    // rabbitMQ service parameters are provided by config
    let ampq_service = &format!(
        "amqp://{}:{}@{}:{}",
        config.rabbit.user, config.rabbit.user, config.rabbit.address, config.rabbit.port,
    );

    // Open connection.
    let mut connection = Connection::insecure_open(ampq_service)?;

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
    exchange.publish(Publish::new(ser_foo.as_bytes(), config.rabbit.queue))?;

    connection.close()
}
