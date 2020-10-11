// This crate provides functions to calculate the relative
// position of the sun and electromagnetic flux

use std::f64::consts::PI;

// number of seconds in a day
const SECONDS_DAY: u64 = 86400;

// days between new year and northern winter solstice
const DAYS_SINCE_SOLSTICE: u64 = 10;

// tilt of earth's axis (this is not a constant)
const OBLIQUITY: f64 = 0.40905;

// average solar flux in space at earth
const SOLAR_CONSTANT: f64 = 1367.0;

const MAX_AIR_MASS: f64 = 40.0;

// The insolation struct provides:
//   flux [W/m^2], instantaneous insolation
//   azimuth angle [deg], the angle from north
//   zenith angle [deg], the angle from zenith
#[derive(Debug)]
pub struct Insolation {
    pub flux: f64,
    pub azimuth: f64,
    pub zenith: f64,
}

impl Insolation {
    pub fn new(day: u64, time: u64, dlat: f64) -> Insolation {
        let (day, time) = time_consistency(day, time);

        // some useful values
        let h = hour_angle(time);

        let lat = dlat.to_radians();

        let dec = declination(day);

        // results
        let zenith = zenith(lat, dec, h);

        let azimuth = azimuth(h, dlat.is_sign_negative());

        let flux = flux(zenith);

        return Insolation {
            flux,
            azimuth: azimuth.to_degrees(),
            zenith: zenith.to_degrees(),
        };
    }
}

fn time_consistency(day: u64, time: u64) -> (u64, u64) {
    // if time [s] is more than a day, add excess time
    // to days and return only time of last day

    let new_day = day + time / SECONDS_DAY;
    let new_time = time % SECONDS_DAY;
    return (new_day, new_time);
}

// calculate the hour angle from elapsed seconds of day
fn hour_angle(time: u64) -> f64 {
    // plus and minus seconds from midd-day
    let noon_time: f64 = time as f64 - (SECONDS_DAY / 2) as f64;

    // angle the earth turns per second
    let per_s = 2.0 * PI / SECONDS_DAY as f64;

    return noon_time * per_s;
}

// declination returns noon zenith angle to the sun for a point
// on the equator; Argument:
//   number of days that have passed in that year
fn declination(d: u64) -> f64 {
    // this value equals obliquity at a solstice
    // and is zero at equinoctes
    let sol_d = (d + DAYS_SINCE_SOLSTICE) as f64;
    return -OBLIQUITY * (2.0 * PI * sol_d / 365.0).cos();
}

// direction to sun, north is 0, east PI/2, south PI, west 3/2 PI
// different on either side of the equator
fn azimuth(h: f64, is_south: bool) -> f64 {
    return if is_south { h } else { h + PI };
}

// zenith returns the zenith angle [rad]
// arguments latitude, declination, hour angle
fn zenith(lat: f64, dec: f64, h: f64) -> f64 {
    let cosz = lat.sin() * dec.sin() + lat.cos() * dec.cos() * h.cos();
    return cosz.acos();
}

// Light attenuation and occlusion

// Air mass is a unitless scalar >1 that models atmospheric attenuation
// higher air mass numbers mean more attenuation
fn air_mass(zenith: f64) -> f64 {
    let secz = 1.0 / zenith.cos();
    return secz.min(MAX_AIR_MASS).max(1.0);
}

// Attenuation by atmosphere
// Approximation by
// Meinel, A. B. and Meinel, M. P. (1976), Applied Solar Energy, Add. Wesl.
// (unconfirmed)
fn attenuation(air_mass: f64) -> f64 {
    return 1.1 * 0.7f64.powf(air_mass.powf(0.678));
}

// No sun when sun is under the horizon
// This is a coarse assumption due to suns angular diametre of about 1 mrad
// and diffraction
fn horizon(zenith: f64) -> f64 {
    return if zenith < PI / 2.0 { 1.0 } else { 0.0 };
}

// flux with atmospheric attenuation and horizon occlusion
fn flux(zenith: f64) -> f64 {
    return horizon(zenith) * SOLAR_CONSTANT * attenuation(air_mass(zenith));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn azimuth_morning() {
        // Azimuth at 9:00, sun is SE
        let insolation = Insolation::new(0, 32400, 40.0);
        let expect = 135.0;

        println!("foo {:?}", insolation);
        assert_eq!(insolation.azimuth, expect);
    }

    #[test]
    fn tropic_at_noon() {
        // Zenith at southern tropic at equinox
        let half_d = SECONDS_DAY / 2;
        let tropic_lat: f64 = -23.43;
        let insolation = Insolation::new(81, half_d, tropic_lat);

        // equinox is is usually not exactly at noon, but within
        // half a day of it, that corresponds to half a degree
        // error in declination
        let e_zenith = tropic_lat.abs();

        println!("foo {:?}", insolation);
        assert!((insolation.zenith - e_zenith).abs() < 0.5);

        // for the same reason air_mass is between 1.085 and 1.095
        assert!(insolation.flux < SOLAR_CONSTANT * attenuation(1.085));
        assert!(insolation.flux > SOLAR_CONSTANT * attenuation(1.095));

        // sun is exactly in north at southern tropic noon
        assert_eq!(insolation.azimuth, 0.0);
    }
}
