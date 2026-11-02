//! PCB thermal and trace design. Writes a Markdown report and an SVG chart of
//! the junction temperature budget to `target/examples/`. Run with
//! `cargo run -p tpt-eng-examples --example pcb_thermal`.

use tpt_eng_examples::{emit, pcb_thermal::run_pcb_thermal};
use tpt_eng_report::{NamedValue, Report, ResultEntry};

fn main() {
    let r = run_pcb_thermal();
    let report = Report::new("PCB thermal and trace design")
        .with_summary(
            "A 2.9 mm microstrip, a 3 mm 2 oz power trace checked for ampacity and \
             resistance, and a 30 W junction temperature on a heat sink.",
        )
        .assumptions(vec![
            NamedValue::new("Substrate height", 1.6).with_unit("mm"),
            NamedValue::new("Relative permittivity", 4.4),
            NamedValue::new("Power dissipation", 30.0).with_unit("W"),
            NamedValue::new("Ambient temperature", 40.0).with_unit("degC"),
        ])
        .results(vec![
            ResultEntry::with_limits(
                "Microstrip impedance",
                r.z0_ohm,
                Some("ohm".into()),
                Some(45.0),
                Some(55.0),
            ),
            ResultEntry::with_limits(
                "Trace ampacity (10 degC rise)",
                r.trace_ampacity_a,
                Some("A".into()),
                None,
                None,
            ),
            ResultEntry::with_limits(
                "Trace resistance (100 mm)",
                r.trace_resistance_ohm,
                Some("ohm".into()),
                None,
                None,
            ),
            ResultEntry::with_limits(
                "Heat-sink resistance",
                r.sink_resistance_k_per_w,
                Some("K/W".into()),
                None,
                None,
            ),
            ResultEntry::with_limits(
                "Junction temperature",
                r.junction_c,
                Some("degC".into()),
                None,
                Some(125.0),
            ),
        ]);
    let bars = vec![
        ("Impedance (ohm)".to_string(), r.z0_ohm),
        ("Ampacity (A)".to_string(), r.trace_ampacity_a),
        ("Junction (degC)".to_string(), r.junction_c),
    ];
    let (md, svg) =
        emit("pcb_thermal", &report, "PCB thermal budget", bars).expect("write outputs");
    println!("report: {}\nchart:  {}", md.display(), svg.display());
}
