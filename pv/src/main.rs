use serde::Deserialize;
use std::env;

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
    let _config = input::Config::new(&args);

    let printer = |x| println!("{:?}", x);
    let _ = consume::receive(&printer);
}
