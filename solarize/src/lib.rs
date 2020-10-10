// The insolation struct provides:
//   flux [W/m^2], instantaneous insolation 
//   azimuth angle [deg], the angle from north
//   zenith angle [deg], the angle from zenith
// 
pub struct Insolation {
    pub flux: f64,
    pub azimuth: f64,
    pub zenith: f64,
}

impl Insolation {
    pub fn new(_day: usize, _time: usize, _lat: usize) -> Insolation {
        let flux = flux();
            let azimuth = azimuth();
            let zenith = zenith();
        return Insolation{ flux, azimuth, zenith }
    }
}

fn flux() -> f64 {
    return 0.0
}

fn azimuth() -> f64 {
    return 0.0
}

fn zenith() -> f64 {
    return 0.0
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
