use serde::Deserialize;
use std::{env, process};
use solarize::Insolation;

pub mod consume;
pub mod input;

#[derive(Deserialize, Debug)]
pub struct Data {
    time: u64,
    power: f64,
}

fn main() {
    // read config from file provided as optional CLI argument
    let args: Vec<String> = env::args().collect();
    let config = input::Config::new(&args).unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1)
    });


    let curried_process = |x: Data| process(150, x.time, 45.0, x.power);
    //let printer = |x| println!("{:?} {}", x, insolation.azimuth );
    let _ = consume::receive(&curried_process, config.rabbit);
}

fn process (day: u64, time: u64, lat: f64,  meter_power: f64 )  {
    let insolation = Insolation::new(day,time,lat);

    // approximation: 1 m^2 panel flat on the ground
    let power = meter_power + insolation.flux * insolation.zenith.to_radians().cos();

    let output = format!("{} {}", time, power);
    println!("{}", output);
}
