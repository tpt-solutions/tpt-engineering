//! Building HVAC load. Writes a Markdown report and an SVG chart of the design
//! loads to `target/examples/`. Run with
//! `cargo run -p tpt-eng-examples --example hvac_load`.

use tpt_eng_examples::{emit, hvac_load::run_hvac_load};
use tpt_eng_report::{NamedValue, Report, ResultEntry};

fn main() {
    let r = run_hvac_load();
    let report = Report::new("Building HVAC load")
        .with_summary(
            "Wall U-value from its layers, whole-envelope conductance, design heating and \
             cooling loads with infiltration, and seasonal heating energy.",
        )
        .assumptions(vec![
            NamedValue::new("Floor area", 120.0).with_unit("m^2"),
            NamedValue::new("Storey height", 3.0).with_unit("m"),
            NamedValue::new("Air changes", 0.5).with_unit("1/h"),
            NamedValue::new("Winter design temperature difference", 25.0).with_unit("K"),
            NamedValue::new("Summer design temperature difference", 8.0).with_unit("K"),
        ])
        .results(vec![
            ResultEntry::with_limits(
                "Wall U-value",
                r.wall_u,
                Some("W/(m^2·K)".into()),
                None,
                Some(1.0),
            ),
            ResultEntry::with_limits(
                "Envelope conductance",
                r.envelope_ua,
                Some("W/K".into()),
                None,
                None,
            ),
            ResultEntry::with_limits(
                "Design heating load",
                r.heating_load_w,
                Some("W".into()),
                None,
                None,
            ),
            ResultEntry::with_limits(
                "Design cooling load",
                r.cooling_load_w,
                Some("W".into()),
                None,
                None,
            ),
            ResultEntry::with_limits(
                "Seasonal heating energy",
                r.annual_heating_kwh,
                Some("kWh".into()),
                None,
                None,
            ),
        ]);
    let bars = vec![
        ("Heating (kW)".to_string(), r.heating_load_w / 1e3),
        ("Cooling (kW)".to_string(), r.cooling_load_w / 1e3),
    ];
    let (md, svg) = emit("hvac_load", &report, "Design loads", bars).expect("write outputs");
    println!("report: {}\nchart:  {}", md.display(), svg.display());
}
