//! Integration tests for `tpt-eng-geotech`.
//!
//! These exercise the public API end-to-end: Mohr-Coulomb strength and factor
//! of safety, the reduced Cam-Clay yield surface and void-ratio update, and
//! borehole stratigraphy summaries.

use tpt_eng_geotech::cam_clay::{ConsolidationMode, cam_clay_yield, void_ratio_update};
use tpt_eng_geotech::mohr_coulomb::{factor_of_safety, shear_strength};
use tpt_eng_geotech::{Borehole, SoilLayer};
use tpt_eng_materials::DataSource;

fn layer(
    top: f64,
    bottom: f64,
    soil_type: &str,
    friction_angle_deg: f64,
    cohesion: f64,
    unit_weight: f64,
) -> SoilLayer {
    SoilLayer {
        depth_top: top,
        depth_bottom: bottom,
        soil_type: soil_type.to_string(),
        friction_angle_deg,
        cohesion,
        unit_weight,
        source: DataSource::standard("test"),
    }
}

#[test]
fn shear_strength_increases_with_normal_stress() {
    let c = 10_000.0;
    let phi = 30.0;
    let low = shear_strength(c, phi, 50_000.0);
    let high = shear_strength(c, phi, 200_000.0);
    assert!(high > low, "strength must grow with normal stress");
    // Reference value: c + 50e3 * tan(30°) ≈ 10e3 + 50e3 * 0.57735
    assert!((low - (c + 50_000.0 * 30.0_f64.to_radians().tan())).abs() < 1e-6);
}

#[test]
fn factor_of_safety_is_one_at_failure() {
    let c = 5_000.0;
    let phi = 25.0;
    let sigma_n = 120_000.0;
    let tau_f = shear_strength(c, phi, sigma_n);
    let fos = factor_of_safety(tau_f, c, phi, sigma_n);
    assert!(
        (fos - 1.0).abs() < 1e-12,
        "FoS at failure should be 1.0, got {fos}"
    );
}

#[test]
fn factor_of_safety_above_one_when_under_stressed() {
    let c = 5_000.0;
    let phi = 25.0;
    let sigma_n = 120_000.0;
    let tau = 0.5 * shear_strength(c, phi, sigma_n);
    let fos = factor_of_safety(tau, c, phi, sigma_n);
    assert!(fos > 1.0, "stable state must have FoS > 1, got {fos}");
}

#[test]
fn cam_clay_yield_is_zero_on_surface_and_positive_outside() {
    let m = 1.0;
    let pc = 200_000.0;
    // On the yield surface: pick p = pc/2 so q² = M²·p·pc/2 > 0.
    let p = pc / 2.0;
    let q = f64::sqrt(m * m * p * pc / 2.0);
    let on_surface = cam_clay_yield(p, q, m, pc);
    assert!(
        on_surface.abs() < 1e-9,
        "yield value on surface should be 0, got {on_surface}"
    );

    // Outside the surface: increase q beyond the yield value.
    let outside = cam_clay_yield(p, q * 1.5, m, pc);
    assert!(
        outside > 0.0,
        "outside state must give positive yield value, got {outside}"
    );

    // Inside the surface (elastic): reduce q to zero with p between 0 and pc.
    let inside = cam_clay_yield(p, 0.0, m, pc);
    assert!(
        inside < 0.0,
        "interior elastic state must give negative value, got {inside}"
    );
}

#[test]
fn void_ratio_update_matches_closed_form() {
    let e0 = 1.0;
    let lambda = 0.2;
    let kappa = 0.05;
    let p0 = 100_000.0;
    let p1 = 200_000.0;

    let normal = void_ratio_update(e0, lambda, kappa, p0, p1, ConsolidationMode::Normal);
    let expected_normal = e0 - lambda * (p1 / p0).ln();
    assert!((normal - expected_normal).abs() < 1e-12);

    let swelling = void_ratio_update(e0, lambda, kappa, p0, p1, ConsolidationMode::Swelling);
    let expected_swelling = e0 - kappa * (p1 / p0).ln();
    assert!((swelling - expected_swelling).abs() < 1e-12);

    // Swelling line is flatter, so for p1 > p0 (compression) normal consolidation
    // reduces e more than swelling would.
    assert!(
        normal < swelling,
        "normal consolidation should reduce e more than swelling"
    );
}

#[test]
fn borehole_total_depth_is_sum_of_layer_thicknesses() {
    let bh = Borehole {
        location: "BH-1".into(),
        layers: vec![
            layer(0.0, 3.0, "topsoil", 20.0, 2_000.0, 18_000.0),
            layer(3.0, 8.0, "sand", 35.0, 0.0, 19_000.0),
            layer(8.0, 12.0, "clay", 22.0, 15_000.0, 20_000.0),
        ],
    };
    assert!((bh.total_depth() - 12.0).abs() < 1e-12);
}

#[test]
fn layer_at_depth_returns_correct_layer() {
    let sand = layer(3.0, 8.0, "sand", 35.0, 0.0, 19_000.0);
    let clay = layer(8.0, 12.0, "clay", 22.0, 15_000.0, 20_000.0);
    let bh = Borehole {
        location: "BH-2".into(),
        layers: vec![
            layer(0.0, 3.0, "topsoil", 20.0, 2_000.0, 18_000.0),
            sand.clone(),
            clay.clone(),
        ],
    };

    // At 5 m we are inside the sand layer.
    let got = bh.layer_at_depth(5.0).expect("layer should exist at 5 m");
    assert_eq!(got.soil_type, "sand");

    // At the top boundary we are inside the layer; just below the bottom is not.
    assert_eq!(bh.layer_at_depth(3.0).unwrap().soil_type, "sand");
    assert!(
        bh.layer_at_depth(8.0).is_none() || bh.layer_at_depth(8.0).unwrap().soil_type == "clay"
    );

    // Below the deepest layer returns None.
    assert!(bh.layer_at_depth(20.0).is_none());
}

#[test]
fn soil_layer_provenance_uses_materials_data_source() {
    let l = layer(0.0, 2.0, "gravel", 40.0, 0.0, 21_000.0);
    assert_eq!(l.source, DataSource::standard("test"));
    assert_eq!(l.thickness(), 2.0);
}
