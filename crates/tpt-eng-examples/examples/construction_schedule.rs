//! Construction project schedule: CPM, PERT, crew leveling and EVM. Writes a
//! Markdown report and an SVG chart to `target/examples/`. Run with
//! `cargo run -p tpt-eng-examples --example construction_schedule`.

use tpt_eng_examples::{construction_schedule::run_construction_schedule, emit};
use tpt_eng_report::{NamedValue, Report, ResultEntry};

fn main() {
    let r = run_construction_schedule();
    let report = Report::new("Construction schedule")
        .with_summary(
            "Critical-path schedule for a small frame project, PERT for the uncertain \
             excavation, crew smoothing, and earned value at a status date.",
        )
        .assumptions(vec![
            NamedValue::new("Budget at completion", 1000.0).with_unit("k"),
            NamedValue::new("Planned value at status date", 800.0).with_unit("k"),
            NamedValue::new("Earned value at status date", 700.0).with_unit("k"),
            NamedValue::new("Actual cost at status date", 750.0).with_unit("k"),
        ])
        .results(vec![
            ResultEntry::with_limits(
                "Project duration",
                r.project_days,
                Some("working days".into()),
                None,
                None,
            ),
            ResultEntry::with_limits(
                "Excavation (PERT expected)",
                r.excavation_pert_days,
                Some("working days".into()),
                None,
                None,
            ),
            ResultEntry::with_limits(
                "Crew peak after smoothing",
                r.crew_peak_after,
                Some("crew".into()),
                None,
                Some(r.crew_peak_before),
            ),
            ResultEntry::with_limits("Cost performance index", r.cpi, None, Some(1.0), None),
            ResultEntry::with_limits("Schedule performance index", r.spi, None, Some(1.0), None),
            ResultEntry::with_limits(
                "Estimate at completion",
                r.eac,
                Some("k".into()),
                None,
                None,
            ),
        ]);
    let bars = vec![
        ("Duration (days)".to_string(), r.project_days),
        ("Crew peak before".to_string(), r.crew_peak_before),
        ("Crew peak after".to_string(), r.crew_peak_after),
    ];
    let (md, svg) =
        emit("construction_schedule", &report, "Schedule and cost", bars).expect("write outputs");
    println!("report: {}\nchart:  {}", md.display(), svg.display());
}
