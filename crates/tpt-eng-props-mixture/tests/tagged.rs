use tpt_eng_props_mixture::{Component, pr_saturation_pressure, pr_saturation_pressure_tagged};
use tpt_eng_provenance::Warnings;

fn water() -> Component {
    Component::from_name("water").unwrap()
}

#[test]
fn tagged_value_matches_untagged_below_critical_temperature() {
    let c = water();
    for t in [280.0, 320.0, 373.15, 450.0, 600.0] {
        let tagged = pr_saturation_pressure_tagged(t, c);
        assert_eq!(tagged.value, pr_saturation_pressure(t, c), "{t} K");
        assert!(
            tagged.provenance.warnings.is_empty(),
            "{t} K should be in range"
        );
    }
}

#[test]
fn validity_bound_is_the_component_critical_temperature() {
    let c = water();
    let tagged = pr_saturation_pressure_tagged(373.15, c);
    let range = tagged.provenance.validity_of("temperature").unwrap();
    assert_eq!((range.min, range.max, range.unit), (0.0, c.tc, "K"));
    assert!(
        tagged
            .provenance
            .source
            .label()
            .starts_with("Peng & Robinson")
    );
}

#[test]
fn at_or_above_critical_temperature_warns_and_keeps_the_core_value() {
    let c = water();
    for t in [c.tc, c.tc + 50.0] {
        let tagged = pr_saturation_pressure_tagged(t, c);
        assert!(
            tagged.provenance.warnings.contains(Warnings::OUT_OF_RANGE),
            "{t} K"
        );
        assert_eq!(tagged.value, pr_saturation_pressure(t, c));
    }
}

#[test]
fn non_finite_temperature_warns() {
    let tagged = pr_saturation_pressure_tagged(f64::NAN, water());
    assert!(tagged.provenance.warnings.contains(Warnings::OUT_OF_RANGE));
}
