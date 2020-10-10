use serde::Deserialize;
use std::{env, process};
use insolation::Insolation;

pub mod consume;
pub mod input;

#[derive(Deserialize, Debug)]
pub struct Data {
    time: usize,
    power: f64,
}

fn main() {
    // read config from file provided as optional CLI argument
    let args: Vec<String> = env::args().collect();
    let config = input::Config::new(&args).unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1)
    });

    let insolation = Insolation::new(0,0,0.0);

    let printer = |x| println!("{:?} {}", x, insolation.azimuth );
    let _ = consume::receive(&printer, config.rabbit);
}
