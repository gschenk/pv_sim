use amiquip::{Connection, ConsumerMessage, ConsumerOptions, QueueDeclareOptions, Result};
use serde::Deserialize;
use serde_json;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct Foo {
    x: f64,
    s: String,
}

fn deser(s: &str) -> Result<Foo, Box<dyn Error>> {
    let foo: Foo = serde_json::from_str(&s)?;
    return Ok(foo);
}

fn main() -> Result<()> {
    // Open connection.
    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;

    // Open a channel - None says let the library choose the channel ID.
    let channel = connection.open_channel(None)?;

    // Declare the "hello" queue.
    let queue = channel.queue_declare("foo", QueueDeclareOptions::default())?;

    // Start a consumer.
    let consumer = queue.consume(ConsumerOptions::default())?;
    println!("Waiting for messages. Press Ctrl-C to exit.");

    for (i, message) in consumer.receiver().iter().enumerate() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let body = String::from_utf8_lossy(&delivery.body);
                let foo = deser(&body);

                // ack only if string was deserialized correctly
                let bar = match foo {
                    Ok(o) => o,
                    Err(err) => {
                        eprintln!("Cannot parse received string: {}", body);
                        eprintln!("Error {}", err);

                        // if not comprehended, nack, and tell rabbitMQ
                        // to remove from message queue
                        consumer.nack(delivery, false)?;
                        continue;
                    }
                };
                consumer.ack(delivery)?;
                println!("{:>3} {:?}", i, bar)
            }
            other => {
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
    }

    connection.close()
}
