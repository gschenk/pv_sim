use serde::Deserialize;
use std::{env, process};

pub mod consume;
pub mod input;
mod power;

#[derive(Deserialize, Debug)]
pub struct Data {
    time: u64,
    power: f64,
}

#[derive(Deserialize, Debug)]
pub struct Panel {
    alignment: f64,   // 180 facing south, 0 facing north [deg]
    inclination: f64, //angle from horizontal [deg]
    peak: f64,        // nominal power of installed PV panels [kW]
    efficiency: f64,  // ratio output to photonic input
}

fn main() {
    // read config from file provided as optional CLI argument
    let args: Vec<String> = env::args().collect();
    let config = input::Config::new(&args).unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1)
    });

    let panel = Panel {
        alignment: 90.0,
        inclination: 30.0,
        peak: 10.0,
        efficiency: 0.15,
    };

    // this closure is going to be passed to rabbit consumer
    // to process each incomming message immediately
    let process = |meter: Data| {
        // Mock time from the meter is passed to the solar function.  This is
        // problmatic in some ways:
        // - real world this would just get the present value as the PV
        //   electronics measures it.
        // - when properly mocking this (and doing so at speed) we ought to use
        //   a common time source (eg mocking an NTSC) since we use asynchronous
        //   messaging
        // - for actual applications the asynchronicity between PV and metering
        //   may be problematic in edge cases where PV from the panels changes
        //   on a fast time-scale eg due to shading or when controlling cos(phi)
        //   as well.

        // calculate solar power of PV
        let solar = power::solar(150, meter.time, 45.0, &panel);

        // format output
        let output = format!("{} {}", meter.time, solar + meter.power);

        // write to STDOUT
        println!("{}", output);
    };
    //let printer = |x| println!("{:?} {}", x, insolation.azimuth );
    let _ = consume::receive(&process, config.rabbit);
}
