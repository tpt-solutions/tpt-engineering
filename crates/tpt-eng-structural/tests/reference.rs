//! Checks `tests/reference/beams.csv`: every row is a closed-form beam result
//! with its citation in the `source` column. The test fails if a row's case is
//! unknown, its quantity is unknown, or the computed value differs by more than
//! 1e-9 relative.

use tpt_eng_structural::{Beam, Load};
use tpt_math_units::uom::si::f64::{Force, Length};
use tpt_math_units::uom::si::{force::kilonewton, length::meter, torque::kilonewton_meter};

const CSV: &str = include_str!("reference/beams.csv");

/// Build the beam named by a CSV `case`.
fn beam_for(case: &str) -> Beam {
    match case {
        "point_mid_10kN_10m" => {
            let mut b = Beam::new(Length::new::<meter>(10.0));
            b.add(Load::point(
                Length::new::<meter>(5.0),
                Force::new::<kilonewton>(10.0),
            ));
            b
        }
        "point_2p5m_10kN_10m" => {
            let mut b = Beam::new(Length::new::<meter>(10.0));
            b.add(Load::point(
                Length::new::<meter>(2.5),
                Force::new::<kilonewton>(10.0),
            ));
            b
        }
        "udl_2kNm_8m" => {
            let mut b = Beam::new(Length::new::<meter>(8.0));
            b.add(Load::uniform(
                Length::new::<meter>(0.0),
                Length::new::<meter>(8.0),
                Force::new::<kilonewton>(16.0),
            ));
            b
        }
        other => panic!("unknown beam case `{other}`"),
    }
}

/// Evaluate `quantity` on `beam`, in the unit the CSV uses.
fn evaluate(beam: &Beam, quantity: &str) -> f64 {
    match quantity {
        "reaction_a" => beam.reaction_a().get::<kilonewton>(),
        "max_moment" => beam.max_bending_moment().get::<kilonewton_meter>(),
        "moment_at_load" => {
            // The load sits at x = 2.5 m for the `point_2p5m` case.
            beam.moment_at(Length::new::<meter>(2.5))
                .get::<kilonewton_meter>()
        }
        other => panic!("unknown quantity `{other}`"),
    }
}

#[test]
fn beams_match_reference_csv() {
    let mut rows = 0;
    for line in CSV.lines().skip(1).filter(|l| !l.trim().is_empty()) {
        let cols: Vec<&str> = line.split(',').collect();
        assert_eq!(cols.len(), 5, "malformed row: {line}");
        let (case, quantity, source) = (cols[0], cols[1], cols[4]);
        let expected: f64 = cols[2].parse().expect("expected value");
        assert!(!source.trim().is_empty(), "row without a citation: {line}");

        let got = evaluate(&beam_for(case), quantity);
        let rel = (got - expected).abs() / expected.abs().max(1e-12);
        assert!(
            rel < 1e-9,
            "{case} {quantity}: got {got}, expected {expected} ({source})"
        );
        rows += 1;
    }
    assert!(rows >= 6, "reference file has only {rows} rows");
}
