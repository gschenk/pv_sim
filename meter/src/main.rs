use amiquip::{Connection, Exchange, Publish, Result};
use serde::Serialize;
use serde_json;

#[derive(Serialize)]
struct Foo {
    x: f64,
    s: String,
}

fn main() -> Result<()> {
    // Open connection.
    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;

    // Open a channel - None says let the library choose the channel ID.
    let channel = connection.open_channel(None)?;

    // Get a handle to the direct exchange on our channel.
    let exchange = Exchange::direct(&channel);

    let foo = Foo {
        x: std::f64::consts::E,
        s: "bar".to_string(),
    };
    let ser_foo = serde_json::to_string(&foo).unwrap();

    // Publish a message to the "hello" queue.
    exchange.publish(Publish::new(ser_foo.as_bytes(), "foo"))?;

    connection.close()
}
