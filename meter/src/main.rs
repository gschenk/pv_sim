use serde::Serialize;
use std::{env, process};

pub mod input;
mod publisher;
mod time;

#[derive(Serialize)]
pub struct Data {
    time: u64,
    power: f64,
    day: u64,
    year: u64,
}

fn main() {
    // read config from filename provided as optional CLI argument
    let args: Vec<String> = env::args().collect();
    let config = input::Config::new(&args).unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1)
    });

    // closure for configuration on rabbitMQ publisher
    let publish = |data| publisher::send(data, &config.rabbit);

    // `time` mocks a proper time source
    let mut time = time::Time::new(&config.time);

    // looping time
    while let Some(now) = time.now() {
        // this struct is going to be sent back
        let data = Data {
            time: now,              // elapsed time in seconds
            power: 0.0,             // power consumption in W
            day: config.time.day,   // start day
            year: config.time.year, // start year
        };
        let _ = &publish(data);
    }
}
