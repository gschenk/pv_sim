use serde::Deserialize;
use std::{env, process};

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
    let _config = input::Config::new(&args).unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1)
    });

    let printer = |x| println!("{:?}", x);
    let _ = consume::receive(&printer);
}
