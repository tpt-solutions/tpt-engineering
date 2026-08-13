//! Example: evaluate a design stress against an allowable limit using the
//! recommended safety factor for a general static application.

use tpt_eng_quantity::Quantity;
use tpt_eng_safety::evaluate_with_class;
use tpt_eng_standards::ApplicationClass;

fn main() {
    let report = evaluate_with_class(
        "beam bending",
        Quantity::pascals(120.0),
        Quantity::pascals(250.0),
        ApplicationClass::StaticGeneral,
    )
    .unwrap();
    println!("{}", report.message);
    println!(
        "utilization = {:.3}, margin = {:.3} (status = {:?})",
        report.utilization, report.margin, report.status
    );
}
