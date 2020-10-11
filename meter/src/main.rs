use serde::Serialize;
use std::{env, process};

pub mod input;
mod publisher;
mod time;


#[derive(Serialize)]
pub struct Data {
    time: u64,
    power: f64,
}

fn main() {
    // read config from filename provided as optional CLI argument
    let args: Vec<String> = env::args().collect();
    let config = input::Config::new(&args).unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1)
    });

    // closure for configuration on rabbitMQ
    let publish = |data| publisher::send(data, &config.rabbit);

    // looping time
    let mut time = time::Time::new(&config.time);
    while let Some(now) = time.now() {
        println!("{:?}", now);
        let data = Data {
            time: now,
            power: std::f64::consts::E,
        };
        let _ = &publish(data);
    }

}
