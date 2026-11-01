//! Mechanical design: section, material, tolerance stack-up, GD&T check and a
//! Markdown calculation report. Run with
//! `cargo run -p tpt-eng-examples --example mechanical_design`.

use tpt_eng_examples::mechanical_design::{design_report, run_mechanical_design};

fn main() {
    let report = run_mechanical_design();
    println!("{}", tpt_eng_report::to_markdown(&design_report(&report)));
}
