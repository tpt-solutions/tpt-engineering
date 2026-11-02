use tpt_eng_props_air::{
    relative_humidity, relative_humidity_tagged, saturation_pressure_water,
    saturation_pressure_water_tagged,
};
use tpt_eng_provenance::Warnings;
use tpt_math_units::uom::si::f64::{Pressure, ThermodynamicTemperature};
use tpt_math_units::uom::si::pressure::pascal;
use tpt_math_units::uom::si::thermodynamic_temperature::kelvin;

fn t(k: f64) -> ThermodynamicTemperature {
    ThermodynamicTemperature::new::<kelvin>(k)
}

fn pa(v: f64) -> Pressure {
    Pressure::new::<pascal>(v)
}

#[test]
fn saturation_tagged_value_matches_untagged_across_grid() {
    for k in (180..=470).step_by(5) {
        let tagged = saturation_pressure_water_tagged(t(k as f64));
        assert_eq!(tagged.value, saturation_pressure_water(t(k as f64)));
        assert!(
            tagged.provenance.warnings.is_empty(),
            "{k} K should be in range"
        );
    }
}

#[test]
fn saturation_outside_range_warns_but_keeps_value() {
    let tagged = saturation_pressure_water_tagged(t(600.0));
    assert!(tagged.provenance.warnings.contains(Warnings::OUT_OF_RANGE));
    assert_eq!(tagged.value, saturation_pressure_water(t(600.0)));
}

#[test]
fn provenance_names_ashrae_and_range() {
    let tagged = saturation_pressure_water_tagged(t(300.0));
    assert!(tagged.provenance.source.label().starts_with("ASHRAE"));
    let range = tagged.provenance.validity_of("temperature").unwrap();
    assert_eq!((range.min, range.max), (173.15, 473.15));
}

#[test]
fn relative_humidity_tagged_matches_untagged() {
    let psat = saturation_pressure_water(t(293.15));
    let tagged = relative_humidity_tagged(pa(1000.0), t(293.15)).unwrap();
    assert_eq!(
        tagged.value,
        relative_humidity(pa(1000.0), t(293.15)).unwrap()
    );
    assert!(tagged.provenance.warnings.is_empty());
    assert!((relative_humidity_tagged(psat, t(293.15)).unwrap().value - 1.0).abs() < 1e-12);
}

#[test]
fn relative_humidity_tagged_out_of_range_warns() {
    let tagged = relative_humidity_tagged(pa(1000.0), t(500.0)).unwrap();
    assert!(tagged.provenance.warnings.contains(Warnings::OUT_OF_RANGE));
    assert_eq!(
        tagged.value,
        relative_humidity(pa(1000.0), t(500.0)).unwrap()
    );
}

#[test]
fn relative_humidity_tagged_keeps_hard_errors() {
    assert!(relative_humidity_tagged(pa(1000.0), t(0.0)).is_err());
}
