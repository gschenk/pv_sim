// This module artfully mocks a clock that returns the present
// time when called with `time.now()`. It simply iterates an
// internal store at each call.
// Parameters are defined in config.toml [time]

use crate::input;

// number of seconds in a day
const SECONDS_HOUR: u64 = 3600;

pub struct Time {
    time: Option<u64>,
    // this is to store a curried function that increases time
    stepper: Box<dyn Fn(Option<u64>) -> Option<u64>>,
}

impl Time {
    pub fn new(config: input::Time) -> Time {
        let time = Some(config.start * SECONDS_HOUR);

        // currying stepper with config
        let stepper = Box::new(timestep(config));
        return Time{ time, stepper }
    }

    // .now() returns the present time
    // Side Effect: it increases Time's time field
    pub fn now(&mut self) -> Option<u64> {
        let time = self.time;
        let stepper = &self.stepper;
        self.time = stepper(time);
        return time;
    }
}

// function prepared for currying
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
