// This module artfully mocks a clock that returns the present
// time when called with `time.now()`. It simply iterates an
// internal store at each call.
// Parameters are defined in config.toml [time]

use crate::input;

// number of seconds in a day
const SECONDS_HOUR: u64 = 3600;
const SECONDS_DAY: u64 = 3600 * 24;

pub struct Time {
    time: Option<u64>,
    // this is to store a curried function that increases time
    stepper: Box<dyn Fn(Option<u64>) -> Option<u64>>,
}

impl Time {
    pub fn new(config: &input::Time) -> Time {
        let time = Some(config.start * SECONDS_HOUR);

        // currying stepper with config (this was difficult!)
        let stepper = Box::new(timestep(config.clone()));
        return Time { time, stepper };
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

// function wiht impl for currying
fn timestep(config: input::Time) -> impl Fn(Option<u64>) -> Option<u64> {
    move |t| {
        let time = match t {
            Some(t) => t,
            _ => return None,
        };
        let next = time + config.stepsize;
        let max = config.end * SECONDS_HOUR;
        return if next <= max { Some(next) } else { None };
    }
}

#[cfg(test)]
mod tests {
    use crate::input;

    #[test]
    fn time_propagates() {
        // mock config
        let config = input::Time {
            stepsize: 1,
            start: 0,
            end: 1,
            day: 156,
            year: 2020,
        };

        let mut time = super::Time::new(&config);

        // closure fn to test optional typed
        let option_assert = |x, e| {
            let now = match x {
                Some(x) => x,
                _ => panic!(),
            };
            assert_eq!(now, e);
        };

        // check if time propagates
        option_assert(time.now(), 0);
        option_assert(time.now(), 1);
        option_assert(time.now(), 2);

        // go to end of interval
        for _i in 2..super::SECONDS_HOUR - 1 {
            let _ = time.now();
        }
        // next one should be Some
        assert!(time.now().is_some());
        // past range should be None
        assert!(time.now().is_none());
    }
}

// returns time of day as a fraction of a whole day
pub fn fractional_day(t: u64) -> f64 {
    return (t % SECONDS_DAY) as f64 / SECONDS_DAY as f64
}
