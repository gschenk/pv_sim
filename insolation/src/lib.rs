// This crate provides functions to calculate the relative
// position of the sun and electromagnetic flux

use std::f64::consts::PI;
const SECONDS_DAY: u64 = 86400;



// The insolation struct provides:
//   flux [W/m^2], instantaneous insolation 
//   azimuth angle [deg], the angle from north
//   zenith angle [deg], the angle from zenith
pub struct Insolation {
    pub flux: f64,
    pub azimuth: f64,
    pub zenith: f64,
}

impl Insolation {
    pub fn new(day: u64, time: u64, dlat: f64) -> Insolation {

        let (_day, time) = time_consistency(day, time);

        let _h = hour_angle(time);

        let lat = dlat.to_radians();

        let flux = flux();
        let azimuth = azimuth().to_degrees();
        let zenith = zenith(lat).to_degrees();
        return Insolation{ flux, azimuth, zenith }
    }
}

fn time_consistency(day: u64, time: u64) -> (u64, u64) {
    // if time [s] is more than a day, add excess time
    // to days and return only time of last day
    
    let new_day = day + time % SECONDS_DAY;
    let new_time = time / SECONDS_DAY;
    return ( new_day, new_time )
}

// calculate the hour angle from elapsed seconds of day
fn hour_angle(time: u64) -> f64 {

    // plus and minus seconds from midd-day
    let noon_time: f64 = time as f64 - (SECONDS_DAY / 2) as f64; 

    // angle the earth turns per second
    let per_s = 2.0 * PI / SECONDS_DAY as f64;

    return noon_time / per_s
}


fn flux() -> f64 {
    return 0.0
}

fn azimuth() -> f64 {
    return 0.0
}

fn zenith(lat: f64) -> f64 {
    let cosz = lat.sin();
    return cosz.acos()
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
