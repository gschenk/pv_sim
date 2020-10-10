use serde::Serialize;
use std::{env, process};

pub mod input;
pub mod publisher;

// number of seconds in a day
const SECONDS_HOUR: u64 = 3600;

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


    // looping time
    let mut time = Time::new(config.time);
    while let Some(i) = time.now() {
        println!("{:?}", i);
    }

    let data = Data {
        time: 123456,
        power: std::f64::consts::E,
    };

    let _ = publisher::send(data, config.rabbit);
}

struct Time {
    time: Option<u64>,
    stepper: Box<dyn Fn(Option<u64>) -> Option<u64>>,
}

impl Time {
    pub fn new(config: input::Time) -> Time {
        let time = Some(config.start * SECONDS_HOUR);

        // currying stepper with config
        let stepper = Box::new(timestep(config));
        return Time{ time, stepper }
    }
    pub fn now(&mut self) -> Option<u64> {
        let time = self.time;
        let stepper = &self.stepper;
        self.time = stepper(time);
        return time;
    }
}

fn timestep(config: input::Time) -> impl Fn(Option<u64>) -> Option<u64> {
    move |t| {
        let time = match t {
            Some(t) => t,
            _ => return None,
        };
        let next = time + config.stepsize;
        let max = config.end * SECONDS_HOUR;
        return if next <= max {
            Some(next)
        } else {
            None
        };
    }
}
