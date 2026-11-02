//! Binary distillation sizing. Writes a Markdown report and an SVG chart of the
//! stage counts to `target/examples/`. Run with
//! `cargo run -p tpt-eng-examples --example distillation`.

use tpt_eng_examples::{distillation::run_distillation, emit};
use tpt_eng_report::{NamedValue, Report, ResultEntry};

fn main() {
    let r = run_distillation();
    let report = Report::new("Binary distillation sizing")
        .with_summary(
            "Fenske, Underwood, Gilliland and McCabe–Thiele sizing for a saturated-liquid \
             feed, with a reboiler effectiveness check.",
        )
        .assumptions(vec![
            NamedValue::new("Relative volatility", r.alpha),
            NamedValue::new("Operating reflux factor", 1.3),
        ])
        .results(vec![
            ResultEntry::with_limits("Minimum stages (Fenske)", r.n_min, None, None, None),
            ResultEntry::with_limits(
                "Minimum reflux ratio (Underwood)",
                r.r_min,
                None,
                None,
                None,
            ),
            ResultEntry::with_limits("Operating reflux ratio", r.reflux, None, None, None),
            ResultEntry::with_limits(
                "Stages (Gilliland)",
                r.stages_gilliland,
                None,
                Some(r.n_min),
                None,
            ),
            ResultEntry::with_limits(
                "Stages (McCabe–Thiele)",
                r.stages_mccabe_thiele,
                None,
                Some(r.n_min),
                None,
            ),
            ResultEntry::with_limits(
                "Reboiler effectiveness",
                r.reboiler_effectiveness,
                None,
                Some(0.0),
                Some(1.0),
            ),
        ]);
    let bars = vec![
        ("Minimum (Fenske)".to_string(), r.n_min),
        ("Gilliland".to_string(), r.stages_gilliland),
        ("McCabe–Thiele".to_string(), r.stages_mccabe_thiele),
    ];
    let (md, svg) =
        emit("distillation", &report, "Theoretical stages", bars).expect("write outputs");
    println!("report: {}\nchart:  {}", md.display(), svg.display());
}
