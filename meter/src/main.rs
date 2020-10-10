use amiquip::{Connection, Exchange, Publish, Result};
use serde::Serialize;
use std::{env, process};

pub mod input;
pub mod publisher;

#[derive(Serialize)]
pub struct Data {
    time: usize,
    power: f64,
}

fn main() {
    // read config from filename provided as optional CLI argument
    let args: Vec<String> = env::args().collect();
    let config = input::Config::new(&args).unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1)
    });
    let data = Data {
        time: 123456,
        power: std::f64::consts::E,
    };

    let _ = publisher::send(data, config.rabbit);
}
