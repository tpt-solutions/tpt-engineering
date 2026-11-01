//! Thermal loop: topology, fuel sizing, PID control, telemetry conditioning and
//! a structural check on the pipe rack. Run with
//! `cargo run -p tpt-eng-examples --example thermal_loop`.

use tpt_eng_examples::thermal_loop::run_thermal_loop;

fn main() {
    let report = run_thermal_loop();
    println!("Thermal loop scenario");
    println!("  nodes                : {}", report.node_count);
    println!("  admittance trace     : {:.3}", report.admittance_trace);
    println!("  setpoint             : {:.1} °C", report.setpoint);
    println!(
        "  supply temperature   : {:.1} °C",
        report.supply_temperature
    );
    println!("  largest gap repaired : {:.1} s", report.max_gap_seconds);
    println!(
        "  rack max moment      : {:.3} kN·m",
        report.rack_max_moment
    );
}
