//! PCB thermal and trace design: a 2 oz outer-layer power trace checked for
//! ampacity and resistance, a 50 Ω microstrip, and the junction temperature of a
//! power device on a heat sink.

use tpt_eng_pcb::{
    ipc_2221_current_capacity, microstrip_impedance, trace_area_mil2, trace_dc_resistance,
};
use tpt_eng_thermal_mgmt::{HeatSink, junction_temperature};

/// Result of the PCB thermal scenario.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PcbThermalReport {
    /// Microstrip characteristic impedance of the 2.9 mm signal trace, Ω.
    pub z0_ohm: f64,
    /// IPC-2221 outer-layer ampacity of the 3 mm, 2 oz power trace at 10 °C
    /// rise, A.
    pub trace_ampacity_a: f64,
    /// DC resistance of a 100 mm run of that trace, Ω.
    pub trace_resistance_ohm: f64,
    /// Heat-sink thermal resistance, K/W.
    pub sink_resistance_k_per_w: f64,
    /// Junction temperature of a 30 W device at 40 °C ambient, °C.
    pub junction_c: f64,
}

/// Evaluate the trace, impedance and thermal budget for a 30 W power stage on
/// 1.6 mm FR-4 (εr = 4.4), 35 µm copper.
///
/// # Panics
///
/// Panics if a PCB or thermal correlation rejects its inputs; the fixed inputs
/// here are valid.
pub fn run_pcb_thermal() -> PcbThermalReport {
    let thickness = 70e-6; // 2 oz copper
    let z0 = microstrip_impedance(2.9e-3, 1.6e-3, 4.4, 35e-6);

    let width = 3.0e-3;
    let area = trace_area_mil2(width, thickness);
    let ampacity = ipc_2221_current_capacity(area, 10.0, true);
    let resistance = trace_dc_resistance(0.100, width, thickness, 1.68e-8);

    // Aluminium heat sink: 40 × 40 mm base, 12 fins 2 mm thick, 15 mm high.
    // Film coefficient 25 W/(m²·K) in forced air; Al k = 200 W/(m·K).
    let sink = HeatSink {
        base_area: 0.040 * 0.040,
        fin_count: 12,
        fin_length: 0.040,
        fin_thickness: 0.002,
        fin_height: 0.015,
    };
    let r_sink = sink.thermal_resistance(25.0, 200.0);

    // Junction-to-case 0.8 K/W and case-to-sink interface 0.3 K/W.
    let theta_ja = 0.8 + 0.3 + r_sink;
    let junction = junction_temperature(30.0, theta_ja, 40.0);

    PcbThermalReport {
        z0_ohm: z0,
        trace_ampacity_a: ampacity,
        trace_resistance_ohm: resistance,
        sink_resistance_k_per_w: r_sink,
        junction_c: junction,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn junction_is_above_ambient_and_ampacity_is_positive() {
        let r = run_pcb_thermal();
        assert!(r.junction_c > 40.0);
        assert!(r.trace_ampacity_a > 0.0);
        assert!(r.trace_resistance_ohm > 0.0);
    }
}
