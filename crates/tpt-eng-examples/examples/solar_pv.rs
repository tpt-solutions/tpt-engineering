//! Solar PV sizing: maximum-power-point sweep and three-phase grid-tie check.
//! Run with `cargo run -p tpt-eng-examples --example solar_pv`.

use tpt_eng_examples::solar_pv::run_solar_pv_sizing;

fn main() {
    let report = run_solar_pv_sizing();
    println!("Solar PV sizing scenario");
    println!("{report:#?}");
}
