use amiquip::{Connection, Exchange, Publish, Result};
use serde::Serialize;
use serde_json;

#[derive(Serialize)]
struct Data {
    time: usize,
    power: f64,
}

fn main() -> Result<()> {
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
