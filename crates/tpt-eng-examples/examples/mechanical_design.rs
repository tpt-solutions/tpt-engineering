//! Mechanical design: section, material, tolerance stack-up, GD&T check and a
//! calculation report. Writes the Markdown report and an SVG chart of the
//! stack-up to `target/examples/`. Run with
//! `cargo run -p tpt-eng-examples --example mechanical_design`.

use tpt_eng_examples::{
    emit, mechanical_design::design_report, mechanical_design::run_mechanical_design,
};

fn main() {
    let r = run_mechanical_design();
    let report = design_report(&r);
    let bars = vec![
        ("Nominal (mm)".to_string(), r.stackup_nominal * 1e3),
        ("Worst-case low (mm)".to_string(), r.stackup_worst_lo * 1e3),
        ("Worst-case high (mm)".to_string(), r.stackup_worst_hi * 1e3),
        ("RSS low (mm)".to_string(), r.stackup_rss_lo * 1e3),
        ("RSS high (mm)".to_string(), r.stackup_rss_hi * 1e3),
    ];
    let (md, svg) =
        emit("mechanical_design", &report, "Stack-up envelope", bars).expect("write outputs");
    println!("report: {}\nchart:  {}", md.display(), svg.display());
}
