//! Provenance-tagged Peng–Robinson saturation pressures and mixture phase
//! boundaries.

use tpt_eng_provenance::{Provenance, Source, Tagged, Validity, Warnings};

use crate::{Component, FixedVec, bubble_point, dew_point, pr_saturation_pressure};

const PENG_ROBINSON: Source =
    Source::Literature("Peng & Robinson (1976), Ind. Eng. Chem. Fundam. 15, 59");

const RAOULT_MIXING: Source = Source::Literature(
    "Ideal-solution (Raoult) mixing of Peng & Robinson (1976) pure-component saturation pressures",
);

/// The temperature range over which a liquid–vapour split exists below `tc`.
fn below_critical(max_tc: f64) -> Validity {
    Validity {
        input: "temperature",
        unit: "K",
        min: 0.0,
        max: max_tc,
    }
}

/// Whether `t` lies strictly below `max_tc`, the critical limit. A non-finite or
/// non-positive temperature is outside the range too.
fn inside(t: f64, max_tc: f64) -> bool {
    t.is_finite() && t > 0.0 && t < max_tc
}

fn warnings_for(t: f64, max_tc: f64) -> Warnings {
    if inside(t, max_tc) {
        Warnings::NONE
    } else {
        Warnings::OUT_OF_RANGE
    }
}

/// Saturation pressure (Pa) of a pure component at `t` (K), with provenance.
///
/// The value is the same as [`pr_saturation_pressure`]. The validity range is
/// below the component's critical temperature, because liquid and vapour only
/// coexist below it. A temperature at or above that limit, or a non-finite one,
/// sets [`Warnings::OUT_OF_RANGE`]. The value is still computed as the untagged
/// function computes it.
pub fn pr_saturation_pressure_tagged(t: f64, c: Component) -> Tagged<f64> {
    Tagged::new(
        pr_saturation_pressure(t, c),
        Provenance::new(PENG_ROBINSON, &[below_critical(c.tc)])
            .with_warnings(warnings_for(t, c.tc)),
    )
}

/// The lowest critical temperature among `components`, or `None` if empty.
fn lowest_tc(components: &[Component]) -> Option<f64> {
    components.iter().map(|c| c.tc).reduce(f64::min)
}

/// Bubble-point pressure (Pa) and vapour fractions of a liquid mixture, with
/// provenance. The value is the same as [`bubble_point`].
///
/// Every component's saturation pressure enters the result, so the validity
/// range is below the lowest critical temperature in the mixture. A temperature
/// at or above it sets [`Warnings::OUT_OF_RANGE`].
pub fn bubble_point_tagged(
    t: f64,
    x: &[f64],
    components: &[Component],
) -> Option<Tagged<(f64, FixedVec<f64>)>> {
    let value = bubble_point(t, x, components)?;
    let max_tc = lowest_tc(components)?;
    Some(Tagged::new(
        value,
        Provenance::new(RAOULT_MIXING, &[below_critical(max_tc)])
            .with_warnings(warnings_for(t, max_tc)),
    ))
}

/// Dew-point pressure (Pa) and liquid fractions of a vapour mixture, with
/// provenance. The value is the same as [`dew_point`]. The validity range and
/// warnings are as for [`bubble_point_tagged`].
pub fn dew_point_tagged(
    t: f64,
    y: &[f64],
    components: &[Component],
) -> Option<Tagged<(f64, FixedVec<f64>)>> {
    let value = dew_point(t, y, components)?;
    let max_tc = lowest_tc(components)?;
    Some(Tagged::new(
        value,
        Provenance::new(RAOULT_MIXING, &[below_critical(max_tc)])
            .with_warnings(warnings_for(t, max_tc)),
    ))
}
