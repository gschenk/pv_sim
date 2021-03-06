use serde::Deserialize;
use std::{env, process};

pub mod consume;
pub mod input;
mod power;
mod timestamp;

#[derive(Deserialize, Debug)]
pub struct Data {
    time: u64,
    day: u64,
    year: u64,
    power: f64,
}

fn main() {
    // read config from file provided as optional CLI argument
    let args: Vec<String> = env::args().collect();
    let config = input::Config::new(&args).unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1)
    });

    if !config.flags.quiet {
        println!("Output: time, Meter [kW], PV [kW], Net [kW]");
    }

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
        let solar = power::solar(meter.day, meter.time, &config.panel);

        // timestamps
        let timestamp = timestamp::from_s_d_y(meter.time, meter.day, meter.year);

        // format output
        let output = format!(
            "{}   {:6.3}  {:6.3}  {:7.3}",
            timestamp,
            meter.power,
            solar,
            solar - meter.power,
        );

        // write to STDOUT
        println!("{}", output);
    };
    //let printer = |x| println!("{:?} {}", x, insolation.azimuth );
    let _ = consume::receive(&process, &config);
}
