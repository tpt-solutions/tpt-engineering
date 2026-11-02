//! Beam statics and section checks (`tpt_eng.structural`).

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use tpt_eng_structural::{Beam, Load, SectionCheck};
use tpt_math_units::uom::si::f64::{Force, Length, Pressure, Torque, Volume};
use tpt_math_units::uom::si::force::newton;
use tpt_math_units::uom::si::length::meter;
use tpt_math_units::uom::si::pressure::pascal;
use tpt_math_units::uom::si::torque::newton_meter;
use tpt_math_units::uom::si::volume::cubic_meter;

/// A simply-supported beam (pin at x = 0, roller at x = span) with transverse
/// loads. All positions are metres from the pin. Downward forces are positive.
#[pyclass(module = "tpt_eng.structural", name = "SimplySupportedBeam")]
pub struct SimplySupportedBeam {
    inner: Beam,
}

fn check_finite(name: &str, value: f64) -> PyResult<()> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(PyValueError::new_err(format!(
            "{name} must be finite, got {value}"
        )))
    }
}

fn check_positive(name: &str, value: f64) -> PyResult<()> {
    check_finite(name, value)?;
    if value <= 0.0 {
        return Err(PyValueError::new_err(format!(
            "{name} must be greater than zero"
        )));
    }
    Ok(())
}

#[pymethods]
impl SimplySupportedBeam {
    #[new]
    fn new(span_m: f64) -> PyResult<Self> {
        check_positive("span_m", span_m)?;
        Ok(Self {
            inner: Beam::new(Length::new::<meter>(span_m)),
        })
    }

    /// Add a downward point load of `force_n` newtons at `at_m` metres.
    fn add_point_load(&mut self, at_m: f64, force_n: f64) -> PyResult<()> {
        self.add(Load::point(
            Length::new::<meter>(at_m),
            Force::new::<newton>(force_n),
        ))
    }

    /// Add a uniform downward load with `total_n` newtons spread over
    /// `[start_m, end_m]`.
    fn add_uniform_load(&mut self, start_m: f64, end_m: f64, total_n: f64) -> PyResult<()> {
        self.add(Load::uniform(
            Length::new::<meter>(start_m),
            Length::new::<meter>(end_m),
            Force::new::<newton>(total_n),
        ))
    }

    /// Add a concentrated moment of `moment_nm` newton-metres at `at_m`
    /// (counter-clockwise positive).
    fn add_moment(&mut self, at_m: f64, moment_nm: f64) -> PyResult<()> {
        self.add(Load::moment(
            Length::new::<meter>(at_m),
            Torque::new::<newton_meter>(moment_nm),
        ))
    }

    /// Vertical reaction at the pin (x = 0), in newtons.
    fn reaction_a_n(&self) -> f64 {
        self.inner.reaction_a().get::<newton>()
    }

    /// Vertical reaction at the roller (x = span), in newtons.
    fn reaction_b_n(&self) -> f64 {
        self.inner.reaction_b().get::<newton>()
    }

    /// Internal shear force at `x_m`, in newtons.
    fn shear_at_n(&self, x_m: f64) -> f64 {
        self.inner
            .shear_at(Length::new::<meter>(x_m))
            .get::<newton>()
    }

    /// Internal bending moment at `x_m`, in newton-metres.
    fn moment_at_nm(&self, x_m: f64) -> f64 {
        self.inner
            .moment_at(Length::new::<meter>(x_m))
            .get::<newton_meter>()
    }

    /// Largest absolute bending moment, in newton-metres. `resolution` is the
    /// number of sample intervals along the span; the default is the core's
    /// default resolution.
    #[pyo3(signature = (resolution=None))]
    fn max_bending_moment_nm(&self, resolution: Option<usize>) -> f64 {
        let m = match resolution {
            Some(n) => self.inner.max_bending_moment_with_resolution(n),
            None => self.inner.max_bending_moment(),
        };
        m.get::<newton_meter>()
    }

    fn __repr__(&self) -> String {
        format!(
            "SimplySupportedBeam(span_m={}, loads={})",
            self.inner.span.get::<meter>(),
            self.inner.loads().len()
        )
    }
}

impl SimplySupportedBeam {
    fn add(&mut self, load: Load) -> PyResult<()> {
        self.inner
            .try_add(load)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

/// Demand/capacity ratio `|M| / (Z · σ_allow)` for an allowable-stress check.
///
/// Values at or below 1.0 pass. Above 1.0 means the section is overstressed.
#[pyfunction]
fn section_utilization(
    moment_nm: f64,
    section_modulus_m3: f64,
    allowable_stress_pa: f64,
) -> PyResult<f64> {
    check_finite("moment_nm", moment_nm)?;
    check_positive("section_modulus_m3", section_modulus_m3)?;
    check_positive("allowable_stress_pa", allowable_stress_pa)?;
    let check = SectionCheck::new(
        Volume::new::<cubic_meter>(section_modulus_m3),
        Pressure::new::<pascal>(allowable_stress_pa),
    );
    Ok(check.utilization(Torque::new::<newton_meter>(moment_nm)))
}

/// Register the structural bindings on the `tpt_eng.structural` submodule.
pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<SimplySupportedBeam>()?;
    m.add_function(wrap_pyfunction!(section_utilization, m)?)?;
    Ok(())
}
