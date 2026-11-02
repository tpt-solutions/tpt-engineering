//! Solar PV sizing: maximum-power-point sweep and three-phase grid-tie check.
//! Writes a Markdown report and an SVG chart to `target/examples/`. Run with
//! `cargo run -p tpt-eng-examples --example solar_pv`.

use tpt_eng_examples::{emit, solar_pv::run_solar_pv_sizing};
use tpt_eng_report::{NamedValue, Report, ResultEntry};

fn main() {
    let r = run_solar_pv_sizing();
    // Real power delivered to the grid by the three-phase tie, kW.
    let grid_kw = 3.0f64.sqrt() * r.grid_v_ll * r.line_current_a * r.power_factor / 1e3;
    let report = Report::new("Solar PV sizing")
        .with_summary(
            "Single-diode maximum-power-point sweep scaled to an array, checked \
             against a three-phase grid tie.",
        )
        .assumptions(vec![
            NamedValue::new("Cells per module", r.cells_per_module as f64),
            NamedValue::new("Modules", r.modules as f64),
            NamedValue::new("Grid voltage (line-line)", r.grid_v_ll).with_unit("V"),
            NamedValue::new("Power factor", r.power_factor),
        ])
        .results(vec![
            ResultEntry::with_limits(
                "Module voltage at MPP",
                r.module_v_mp,
                Some("V".into()),
                None,
                None,
            ),
            ResultEntry::with_limits(
                "Module current at MPP",
                r.module_i_mp,
                Some("A".into()),
                None,
                None,
            ),
            ResultEntry::with_limits(
                "Array DC power",
                r.array_dc_w / 1e3,
                Some("kW".into()),
                None,
                None,
            ),
            ResultEntry::with_limits("Grid AC power", grid_kw, Some("kW".into()), None, None),
        ]);
    let bars = vec![
        ("Array DC (kW)".to_string(), r.array_dc_w / 1e3),
        ("Grid AC (kW)".to_string(), grid_kw),
    ];
    let (md, svg) = emit("solar_pv", &report, "Solar PV power", bars).expect("write outputs");
    println!("report: {}\nchart:  {}", md.display(), svg.display());
}
