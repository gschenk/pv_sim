use amiquip::{Connection, Exchange, Publish, Result};
use serde_json;

use crate::input::Rabbit;
use crate::Data;

pub fn send(data: Data, config: Rabbit) -> Result<()> {
    // rabbitMQ service parameters are provided by config
    let ampq_service = &format!(
        "amqp://{}:{}@{}:{}",
        config.user, config.user, config.address, config.port,
    );

    // Open connection.
    let mut connection = Connection::insecure_open(ampq_service)?;

    // Open a channel - None says let the library choose the channel ID.
    let channel = connection.open_channel(None)?;

    // Get a handle to the direct exchange on our channel.
    let exchange = Exchange::direct(&channel);

    let ser_foo = serde_json::to_string(&data).unwrap();

    // Publish a message to the "hello" queue.
    exchange.publish(Publish::new(ser_foo.as_bytes(), config.queue))?;

    connection.close()
}
