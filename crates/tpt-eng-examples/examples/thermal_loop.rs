//! Thermal loop: topology, fuel sizing, PID control, telemetry conditioning and
//! a structural check on the pipe rack. Writes a Markdown report and an SVG
//! chart to `target/examples/`. Run with
//! `cargo run -p tpt-eng-examples --example thermal_loop`.

use tpt_eng_examples::{emit, thermal_loop::run_thermal_loop};
use tpt_eng_report::{NamedValue, Report, ResultEntry};

fn main() {
    let r = run_thermal_loop();
    let report = Report::new("Thermal loop")
        .with_summary(
            "Hydraulic topology, fuel-sized burner, PID supply control on conditioned \
             telemetry, and a beam check on the pipe rack.",
        )
        .assumptions(vec![
            NamedValue::new("Setpoint", r.setpoint).with_unit("degC"),
            NamedValue::new("Loop nodes", r.node_count as f64),
        ])
        .results(vec![
            ResultEntry::with_limits(
                "Supply temperature",
                r.supply_temperature,
                Some("degC".into()),
                None,
                None,
            ),
            ResultEntry::with_limits(
                "Largest gap repaired",
                r.max_gap_seconds,
                Some("s".into()),
                None,
                None,
            ),
            ResultEntry::with_limits(
                "Rack max bending moment",
                r.rack_max_moment,
                Some("kN·m".into()),
                None,
                None,
            ),
        ]);
    let bars = vec![
        ("Setpoint (°C)".to_string(), r.setpoint),
        ("Supply (°C)".to_string(), r.supply_temperature),
    ];
    let (md, svg) =
        emit("thermal_loop", &report, "Thermal loop temperatures", bars).expect("write outputs");
    println!("report: {}\nchart:  {}", md.display(), svg.display());
}
