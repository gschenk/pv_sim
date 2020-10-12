use serde::Serialize;
use std::{env, process};

pub mod input;
mod publisher;
mod random;
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

    // random walk parameters:
    // sigma has to scale with step size
    let sigma: f64 = config.random.sigma * (config.time.stepsize as f64).sqrt();

    // random walk needs power from previous iteration
    let mut power = config.random.min_power;

    // looping time
    while let Some(now) = time.now() {
        // this struct is going to be sent back

        power = random::simulator(
            time::fractional_day(now),
            power,
            config.random.min_power,
            config.random.max_power,
            sigma,
        );
        let data = Data {
            time: now,              // elapsed time in seconds
            power,                  // power consumption in W
            day: config.time.day,   // start day
            year: config.time.year, // start year
        };
        let _ = &publish(data);
    }
}
