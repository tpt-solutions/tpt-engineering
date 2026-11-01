//! Trace models: microstrip characteristic impedance and IPC-2221 current
//! capacity, plus cross-section / DC-resistance helpers.

use tpt_eng_electrical::dc_resistance;

use crate::MIL_TO_M;

/// Cross-sectional area of a rectangular trace (mils²) from its width and
/// thickness (both in metres).
///
/// ```text
/// A[mil²] = (w[m] / 2.54e-5) · (t[m] / 2.54e-5)
/// ```
///
/// IPC-2221 current-capacity relations are expressed in mils², so this helper
/// bridges SI trace geometry to those formulas.
pub fn trace_area_mil2(width_m: f64, thickness_m: f64) -> f64 {
    let w_mil = width_m / MIL_TO_M;
    let t_mil = thickness_m / MIL_TO_M;
    w_mil * t_mil
}

/// Characteristic impedance (Ω) of a surface microstrip.
///
/// Uses the widely cited closed-form approximations (Wheeler / IPC for narrow
/// strips, Edwards for wide strips), all in SI units:
///
/// * `w/h ≤ 1` (narrow):
///   ```text
///   Z0 = (87 / √(εᵣ + 1.41)) · ln( 5.98·h / (0.8·w + t) )
///   ```
/// * `w/h > 1` (wide, Edwards):
///   ```text
///   Z0 = (60 / √(εᵣ + 1.41)) · ln( 8·h/w + 0.25·w/h )
///   ```
///
/// where `w` is the trace width (m), `h` the substrate height beneath the trace
/// (m), `t` the trace thickness (m) and `εᵣ` the substrate relative
/// permittivity.
///
/// # Arguments
///
/// * `width_m` — trace width (m).
/// * `height_m` — substrate height between the trace and the reference plane (m).
/// * `relative_permittivity` — substrate relative permittivity `εᵣ` (dimensionless).
/// * `trace_thickness_m` — conductor thickness (m).
///
/// Uses the Hammerstad–Jensen closed form with Wheeler's finite-thickness
/// width correction. The two branches (`w/h ≤ 1` and `w/h > 1`) agree at
/// `w/h = 1`, so the impedance is continuous across the boundary.
///
/// # Panics
///
/// Panics if `width_m`, `height_m` or `relative_permittivity` is not finite and
/// strictly positive, or if `trace_thickness_m` is negative or non-finite.
pub fn microstrip_impedance(
    width_m: f64,
    height_m: f64,
    relative_permittivity: f64,
    trace_thickness_m: f64,
) -> f64 {
    assert!(
        width_m.is_finite() && width_m > 0.0,
        "trace width must be positive"
    );
    assert!(
        height_m.is_finite() && height_m > 0.0,
        "substrate height must be positive"
    );
    assert!(
        relative_permittivity.is_finite() && relative_permittivity > 0.0,
        "relative permittivity must be positive"
    );
    assert!(
        trace_thickness_m.is_finite() && trace_thickness_m >= 0.0,
        "trace thickness must be non-negative"
    );

    // Wheeler's thickness correction widens the effective strip: Δw = (t/π)(1 + ln(2h/t)).
    let dw_over_h = if trace_thickness_m > 0.0 {
        trace_thickness_m / (std::f64::consts::PI * height_m)
            * (1.0 + (2.0 * height_m / trace_thickness_m).ln())
    } else {
        0.0
    };
    let u = width_m / height_m + dw_over_h;

    let er = relative_permittivity;
    let er_eff = (er + 1.0) / 2.0 + (er - 1.0) / 2.0 / (1.0 + 12.0 / u).sqrt();

    if u <= 1.0 {
        60.0 / er_eff.sqrt() * (8.0 / u + u / 4.0).ln()
    } else {
        let z_free = 120.0 * std::f64::consts::PI / (u + 1.393 + 0.667 * (u + 1.444).ln());
        z_free / er_eff.sqrt()
    }
}

/// Maximum permissible current (A) of a trace per the IPC-2221 external/internal
/// temperature-rise nomograph.
///
/// ```text
/// I = k · ΔT^0.44 · A^0.725
/// ```
///
/// with `A` the cross-sectional area in **mils²** and `ΔT` the allowed
/// temperature rise (°C). The coefficient is `k = 0.048` for outer (external)
/// layers and `k = 0.024` for inner (internal) layers.
///
/// # Arguments
///
/// * `area_mil2` — cross-sectional area (mils²). Use [`trace_area_mil2`] to
///   convert SI geometry, or supply a value already in mils².
/// * `temp_rise_c` — allowed temperature rise above ambient (°C).
/// * `external` — `true` for an outer-layer trace, `false` for an inner-layer
///   trace.
///
/// # Panics
///
/// Panics if `area_mil2` or `temp_rise_c` is negative (non-physical inputs would
/// produce `NaN`).
pub fn ipc_2221_current_capacity(area_mil2: f64, temp_rise_c: f64, external: bool) -> f64 {
    assert!(
        area_mil2 >= 0.0,
        "cross-sectional area must be non-negative"
    );
    assert!(temp_rise_c >= 0.0, "temperature rise must be non-negative");
    let k = if external { 0.048 } else { 0.024 };
    k * temp_rise_c.powf(0.44) * area_mil2.powf(0.725)
}

/// DC resistance (Ω) of a straight rectangular trace of given length.
///
/// Combines the trace cross-section (`width_m · thickness_m`) with
/// [`tpt_eng_electrical::dc_resistance`] (`R = ρ·L / A`), so the copper
/// resistivity can be taken straight from `tpt-eng-electrical`'s material
/// tables.
///
/// # Arguments
///
/// * `length_m` — trace length (m).
/// * `width_m` — trace width (m).
/// * `thickness_m` — conductor thickness (m).
/// * `resistivity_ohm_m` — conductor resistivity (Ω·m).
///
/// # Panics
///
/// Panics if the cross-sectional area (`width_m · thickness_m`) is zero.
pub fn trace_dc_resistance(
    length_m: f64,
    width_m: f64,
    thickness_m: f64,
    resistivity_ohm_m: f64,
) -> f64 {
    let area = width_m * thickness_m;
    dc_resistance(length_m, area, resistivity_ohm_m)
}
