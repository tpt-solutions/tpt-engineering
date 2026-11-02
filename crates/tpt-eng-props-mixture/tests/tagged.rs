use tpt_eng_props_mixture::{
    Component, bubble_point, bubble_point_tagged, dew_point, dew_point_tagged,
    pr_saturation_pressure, pr_saturation_pressure_tagged,
};
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

#[test]
fn bubble_point_tagged_matches_untagged_and_is_flagged_above_lowest_tc() {
    let water = Component::from_name("water").unwrap();
    let methane = Component::from_name("methane").unwrap();
    let components = [water, methane];
    let x = [0.2, 0.8];

    // Methane's critical temperature (~190.6 K) is the lowest, so the mixture
    // is only in range below it.
    let inside = bubble_point_tagged(150.0, &x, &components).unwrap();
    assert_eq!(inside.value, bubble_point(150.0, &x, &components).unwrap());
    assert!(inside.provenance.warnings.is_empty());

    let outside = bubble_point_tagged(300.0, &x, &components).unwrap();
    assert!(outside.provenance.warnings.contains(Warnings::OUT_OF_RANGE));
    assert_eq!(outside.value, bubble_point(300.0, &x, &components).unwrap());
    let range = outside.provenance.validity_of("temperature").unwrap();
    assert_eq!(range.max, methane.tc);
}

#[test]
fn dew_point_tagged_matches_untagged() {
    let water = Component::from_name("water").unwrap();
    let methane = Component::from_name("methane").unwrap();
    let components = [water, methane];
    let y = [0.1, 0.9];
    let tagged = dew_point_tagged(150.0, &y, &components).unwrap();
    assert_eq!(tagged.value, dew_point(150.0, &y, &components).unwrap());
    assert!(tagged.provenance.warnings.is_empty());
}

#[test]
fn empty_mixture_returns_none_as_the_untagged_function_does() {
    assert!(bubble_point_tagged(250.0, &[], &[]).is_none());
    assert!(dew_point_tagged(250.0, &[], &[]).is_none());
}
