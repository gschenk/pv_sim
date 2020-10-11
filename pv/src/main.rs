use chrono::{Duration, NaiveDate, NaiveDateTime};
use serde::Deserialize;
use std::convert::TryInto;
use std::{env, process};

pub mod consume;
pub mod input;
mod power;

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
        let solar = power::solar(meter.day, meter.time, 45.0, &config.panel);

        // timestamps
        let timestamp = seconds_to_timestamp(meter.time, meter.day, meter.year);

        // format output
        let output = format!("{} {}", timestamp, solar + meter.power);

        // write to STDOUT
        println!("{}", output);
    };
    //let printer = |x| println!("{:?} {}", x, insolation.azimuth );
    let _ = consume::receive(&process, &config);
}

// calculates iso timestamp for time s, day, year
fn seconds_to_timestamp(seconds: u64, day: u64, year: u64) -> NaiveDateTime{
    let iyear: i32 = year.try_into().unwrap();
    let iday: i64 = day.try_into().unwrap();
    let iseconds: i64 = seconds.try_into().unwrap();

    let date: NaiveDateTime = NaiveDate::from_ymd(iyear, 1, 1)
        .and_hms(0, 0, 0)
        .checked_add_signed(Duration::days(iday))
        .unwrap()
        .checked_add_signed(Duration::seconds(iseconds))
        .unwrap();
    return date;
}
