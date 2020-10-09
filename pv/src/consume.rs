use amiquip::{Connection, ConsumerMessage, ConsumerOptions, QueueDeclareOptions, Result};
//use serde::Deserialize;
use crate::Data;
use serde_json;
use std::error::Error;

// deserialize received data
fn deser(s: &str) -> Result<Data, Box<dyn Error>> {
    let d: Data = serde_json::from_str(&s)?;
    return Ok(d);
}
//fn higer_order_fn<F>(value:i32, step: F)  -> i32
//                    where F: Fn(i32) -> i32 {
//    step(value)
//}
pub fn receive(procedure: &dyn Fn(Data) -> ()) -> Result<()> {
    // Open connection.
    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;

    // Open a channel - None says let the library choose the channel ID.
    let channel = connection.open_channel(None)?;

    // Declare the "hello" queue.
    let queue = channel.queue_declare("foo", QueueDeclareOptions::default())?;

    // Start a consumer.
    let consumer = queue.consume(ConsumerOptions::default())?;
    println!("Waiting for messages. Press Ctrl-C to exit.");

    for (_i, message) in consumer.receiver().iter().enumerate() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let body = String::from_utf8_lossy(&delivery.body);

                // ack only if string was deserialized correctly
                let data = match deser(&body) {
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
                procedure(data);
            }
            other => {
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
    }

    connection.close()
}
