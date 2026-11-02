//! Provenance-tagged Peng–Robinson saturation pressure.

use tpt_eng_provenance::{Provenance, Source, Tagged, Validity, Warnings};

use crate::{Component, pr_saturation_pressure};

const SOURCE: Source = Source::Literature("Peng & Robinson (1976), Ind. Eng. Chem. Fundam. 15, 59");

/// Saturation pressure (Pa) of a pure component at `t` (K), with provenance.
///
/// The value is the same as [`pr_saturation_pressure`]. The validity range is
/// below the component's critical temperature, because liquid and vapour only
/// coexist below it. A temperature at or above that limit, or a non-finite one,
/// sets [`Warnings::OUT_OF_RANGE`]. The value is still computed as the untagged
/// function computes it.
pub fn pr_saturation_pressure_tagged(t: f64, c: Component) -> Tagged<f64> {
    let range = Validity {
        input: "temperature",
        unit: "K",
        min: 0.0,
        max: c.tc,
    };
    let inside = t.is_finite() && t > 0.0 && t < c.tc;
    let warnings = if inside {
        Warnings::NONE
    } else {
        Warnings::OUT_OF_RANGE
    };
    Tagged::new(
        pr_saturation_pressure(t, c),
        Provenance::new(SOURCE, &[range]).with_warnings(warnings),
    )
}
