use serde::Deserialize;
use solarize::Insolation;
use std::{env, process};

pub mod consume;
pub mod input;

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

    let curried_process = |x: Data| process(150, x.time, 45.0, x.power, &panel);
    //let printer = |x| println!("{:?} {}", x, insolation.azimuth );
    let _ = consume::receive(&curried_process, config.rabbit);
}

fn process(day: u64, time: u64, lat: f64, meter_power: f64, panel: &Panel) {
    let insolation = Insolation::new(day, time, lat);

    let area = area_from_peak_power(&panel);

    // approximation: 1 m^2 panel
    let power = meter_power + area * flux_on_panel(insolation, &panel) * panel.efficiency;

    let output = format!("{} {}", time, power);
    println!("{}", output);
}

fn flux_on_panel(insolation: Insolation, panel: &Panel) -> f64 {
    //sun angles:
    let zenith = insolation.zenith.to_radians();
    let azimuth = insolation.azimuth.to_radians();

    //panel angles:
    let inclination = panel.inclination.to_radians();
    let alignment = panel.alignment.to_radians();
    let cross_section = zenith.sin() * inclination.sin() * (alignment - azimuth).cos()
        + zenith.cos() * inclination.cos();
    return cross_section.max(0.0) * insolation.flux;
}

// returns size of panels in [m^2]
fn area_from_peak_power(panel: &Panel) -> f64 {
    // peak power is sometimes defined at nominal air mass values (eg 1.5)
    const ASTM_SOLAR_CONSTANT: f64 = 1000.4; //ASTM G-173
    let converted_flux = ASTM_SOLAR_CONSTANT * panel.efficiency;
    let power = 1e3 * panel.peak; // from kW to W
    return power / converted_flux;
}
