// Module Power provides power calculations, in particular calculating
// output by the PV panels
use crate::Panel;
use solarize::Insolation;

// calculates solar power
pub fn solar(day: u64, time: u64, lat: f64, panel: &Panel) -> f64 {
    let insolation = Insolation::new(day, time, lat);

    let area = area_from_peak_power(&panel);

    // approximation: 1 m^2 panel
    let power = area * flux_on_panel(insolation, &panel) * panel.efficiency;
    return power;
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
