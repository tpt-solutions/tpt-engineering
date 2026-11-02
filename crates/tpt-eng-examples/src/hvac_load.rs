//! Building HVAC load: envelope conductance from wall assemblies, design heating
//! and cooling loads including infiltration, and the plant's operating energy.

use tpt_eng_building_sys::{
    AssemblyLayer, annual_heating_energy_kwh, assembly_u_value, cooling_load, envelope_ua,
    heating_load, infiltration_loss_ach_w,
};

/// Result of the HVAC load scenario.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HvacReport {
    /// Opaque wall assembly U-value, W/(m²·K), from the layer stack.
    pub wall_u: f64,
    /// Whole-envelope conductance, W/K.
    pub envelope_ua: f64,
    /// Design heating load, W (winter design ΔT = 25 K).
    pub heating_load_w: f64,
    /// Design cooling load, W (summer design ΔT = 8 K, plus solar gains).
    pub cooling_load_w: f64,
    /// Heating energy over a season of 2 500 K·h of degree-hours, kWh.
    pub annual_heating_kwh: f64,
}

/// Size a 120 m² single-storey office: a 3 m-high volume, a 2-layer wall, and a
/// roof and glazing in the envelope. Air change rate 0.5 h⁻¹.
///
/// # Panics
///
/// Panics if a building-services correlation rejects its inputs; the fixed
/// inputs here are valid.
pub fn run_hvac_load() -> HvacReport {
    // Wall: 200 mm concrete block (k = 0.8 W/m·K) plus 50 mm insulation
    // (k = 0.035 W/m·K); film resistances interior 0.13, exterior 0.04 m²K/W.
    let wall = [
        AssemblyLayer::new(0.8, 0.20),
        AssemblyLayer::new(0.035, 0.05),
    ];
    let wall_u = assembly_u_value(&wall, 0.13, 0.04);

    let envelope = [
        (60.0, wall_u), // walls, m² and W/(m²·K)
        (120.0, 0.25),  // roof
        (12.0, 2.8),    // double glazing
    ];
    let ua = envelope_ua(&envelope);

    let volume = 120.0 * 3.0;
    let air_density = 1.2;
    let cp_air = 1005.0;
    let heat_infil = infiltration_loss_ach_w(0.5, volume, air_density, cp_air, 25.0);
    let cool_infil = infiltration_loss_ach_w(0.5, volume, air_density, cp_air, 8.0);

    let heating = heating_load(ua, 25.0, heat_infil, 400.0);
    let cooling = cooling_load(ua, 8.0, cool_infil, 900.0);
    let annual = annual_heating_energy_kwh(heating, 2500.0);

    HvacReport {
        wall_u,
        envelope_ua: ua,
        heating_load_w: heating,
        cooling_load_w: cooling,
        annual_heating_kwh: annual,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wall_and_loads_are_physical() {
        let r = run_hvac_load();
        assert!(
            r.wall_u > 0.0 && r.wall_u < 1.0,
            "insulated wall U = {}",
            r.wall_u
        );
        assert!(r.heating_load_w > 0.0 && r.annual_heating_kwh > 0.0);
    }
}
