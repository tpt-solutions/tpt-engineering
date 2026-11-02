//! Control-systems primitives (`tpt_eng.controls`).

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use tpt_eng_controls::{Pid as CorePid, PidGains};

/// Discrete-time PID controller with derivative-on-measurement and
/// conditional-integration anti-windup.
///
/// Give both `output_min` and `output_max` to saturate the output, or neither.
#[pyclass(module = "tpt_eng.controls", name = "Pid")]
pub struct Pid {
    inner: CorePid,
}

#[pymethods]
impl Pid {
    #[new]
    #[pyo3(signature = (kp, ki=0.0, kd=0.0, output_min=None, output_max=None))]
    fn new(
        kp: f64,
        ki: f64,
        kd: f64,
        output_min: Option<f64>,
        output_max: Option<f64>,
    ) -> PyResult<Self> {
        let inner = CorePid::new(PidGains::new(kp, ki, kd));
        let inner = match (output_min, output_max) {
            (None, None) => inner,
            (Some(lo), Some(hi)) if lo <= hi => inner.with_output_limits(lo, hi),
            (Some(_), Some(_)) => {
                return Err(PyValueError::new_err(
                    "output_min must not exceed output_max",
                ));
            }
            _ => {
                return Err(PyValueError::new_err(
                    "give both output_min and output_max, or neither",
                ));
            }
        };
        Ok(Self { inner })
    }

    /// Set the target value the measurement should track.
    fn set_setpoint(&mut self, setpoint: f64) {
        self.inner.set_setpoint(setpoint);
    }

    /// Advance one control step and return the controller output.
    ///
    /// `measurement` is the plant output now, and `dt_s` is the sample period
    /// in seconds, which must be positive.
    fn update(&mut self, measurement: f64, dt_s: f64) -> PyResult<f64> {
        // The core asserts on dt <= 0. Check here so Python gets a ValueError
        // rather than a PanicException. NaN must be rejected explicitly.
        if dt_s.is_nan() || dt_s <= 0.0 {
            return Err(PyValueError::new_err("dt_s must be greater than zero"));
        }
        Ok(self.inner.update(measurement, dt_s))
    }

    /// The current integral accumulator, for diagnostics or bumpless retuning.
    #[getter]
    fn integral(&self) -> f64 {
        self.inner.integral()
    }
}

/// Register the controls bindings on the `tpt_eng.controls` submodule.
pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Pid>()?;
    Ok(())
}
