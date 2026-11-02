use tpt_eng_props_water::{
    Error, Region, saturation_pressure, saturation_pressure_tagged, state, state_tagged,
};
use tpt_eng_provenance::Warnings;
use tpt_math_units::uom::si::f64::{Pressure, ThermodynamicTemperature};
use tpt_math_units::uom::si::pressure::pascal;
use tpt_math_units::uom::si::thermodynamic_temperature::kelvin;

fn t(k: f64) -> ThermodynamicTemperature {
    ThermodynamicTemperature::new::<kelvin>(k)
}

fn p(pa: f64) -> Pressure {
    Pressure::new::<pascal>(pa)
}

#[test]
fn saturation_tagged_value_matches_untagged_across_grid() {
    for k in (280..=640).step_by(10) {
        let tagged = saturation_pressure_tagged(t(k as f64));
        assert_eq!(tagged.value, saturation_pressure(t(k as f64)));
        assert!(
            tagged.provenance.warnings.is_empty(),
            "{k} K should be in range"
        );
    }
}

#[test]
fn saturation_outside_range_warns_but_keeps_value() {
    let tagged = saturation_pressure_tagged(t(700.0));
    assert!(tagged.provenance.warnings.contains(Warnings::OUT_OF_RANGE));
    assert_eq!(tagged.value, saturation_pressure(t(700.0)));
}

#[test]
fn saturation_provenance_names_standard_and_range() {
    let tagged = saturation_pressure_tagged(t(300.0));
    assert_eq!(tagged.provenance.source.label(), "IAPWS-IF97 (2007)");
    let range = tagged.provenance.validity_of("temperature").unwrap();
    assert_eq!((range.min, range.max, range.unit), (273.15, 647.096, "K"));
}

#[test]
fn state_tagged_in_range_matches_untagged() {
    let tagged = state_tagged(t(300.0), p(3.0e6)).unwrap();
    let plain = state(t(300.0), p(3.0e6)).unwrap();
    assert_eq!(tagged.value, plain);
    assert_eq!(tagged.value.region, Region::One);
    assert!(tagged.provenance.warnings.is_empty());
}

#[test]
fn state_tagged_above_range_warns_and_matches_untagged() {
    let tagged = state_tagged(t(1100.0), p(1.0e5)).unwrap();
    assert!(tagged.provenance.warnings.contains(Warnings::OUT_OF_RANGE));
    assert_eq!(tagged.value, state(t(1100.0), p(1.0e5)).unwrap());
}

#[test]
fn state_tagged_pressure_above_range_warns() {
    let tagged = state_tagged(t(300.0), p(120.0e6)).unwrap();
    assert!(tagged.provenance.warnings.contains(Warnings::OUT_OF_RANGE));
}

#[test]
fn state_tagged_keeps_hard_errors() {
    assert_eq!(
        state_tagged(t(200.0), p(1.0e5)).unwrap_err(),
        Error::TemperatureOutOfRange
    );
    assert_eq!(
        state_tagged(t(300.0), p(-1.0)).unwrap_err(),
        Error::NegativePressure
    );
}
