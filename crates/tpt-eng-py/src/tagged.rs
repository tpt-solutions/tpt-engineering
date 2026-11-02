//! Provenance-tagged results (`tpt_eng.Tagged`).
//!
//! A `*_tagged` function returns a [`Tagged`] object. It carries the same value
//! as the untagged function, plus the source, the validity ranges, and a list
//! of warnings for inputs outside those ranges. With `strict=True`, a warning
//! raises `ValueError` instead.

use pyo3::IntoPyObjectExt;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use tpt_eng_provenance::{Provenance, Warnings, out_of_range};

/// A computed value with its provenance.
///
/// * `value`: the result, the same as the untagged function returns.
/// * `source`: the published reference the correlation comes from.
/// * `validity`: tuples of `(input, unit, min, max)` for each checked input.
/// * `warnings`: one message per input outside its range. Empty when every
///   input is in range.
/// * `in_range`: `True` when `warnings` is empty.
#[pyclass(module = "tpt_eng", name = "Tagged", frozen)]
pub struct Tagged {
    value: Py<PyAny>,
    source: &'static str,
    validity: Vec<(&'static str, &'static str, f64, f64)>,
    warnings: Vec<String>,
}

#[pymethods]
impl Tagged {
    #[getter]
    fn value(&self, py: Python<'_>) -> Py<PyAny> {
        self.value.clone_ref(py)
    }

    #[getter]
    fn source(&self) -> &'static str {
        self.source
    }

    #[getter]
    fn validity(&self) -> Vec<(&'static str, &'static str, f64, f64)> {
        self.validity.clone()
    }

    #[getter]
    fn warnings(&self) -> Vec<String> {
        self.warnings.clone()
    }

    #[getter]
    fn in_range(&self) -> bool {
        self.warnings.is_empty()
    }

    fn __repr__(&self) -> String {
        format!(
            "Tagged(source={:?}, in_range={})",
            self.source,
            self.in_range()
        )
    }
}

/// Reject NaN and infinite inputs. The core functions would return NaN for
/// them, and that should never reach Python as a silent result.
pub fn require_finite(name: &str, value: f64) -> PyResult<()> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(PyValueError::new_err(format!(
            "{name} must be finite, got {value}"
        )))
    }
}

/// Require a finite, strictly positive value, such as an absolute temperature.
pub fn require_positive(name: &str, value: f64) -> PyResult<()> {
    require_finite(name, value)?;
    if value > 0.0 {
        Ok(())
    } else {
        Err(PyValueError::new_err(format!(
            "{name} must be greater than zero, got {value}"
        )))
    }
}

/// Build a [`Tagged`] result from a core `value` and its `provenance`.
///
/// `inputs` pairs each checked input name with its value, matching a
/// `Validity::input` in the provenance. An input outside its range adds a
/// warning. With `strict`, any warning raises `ValueError`.
pub fn finish<'py, T>(
    py: Python<'py>,
    value: T,
    provenance: Provenance,
    inputs: &[(&str, f64)],
    strict: bool,
) -> PyResult<Tagged>
where
    T: IntoPyObject<'py>,
{
    let mut warnings = Vec::new();
    for &(name, v) in inputs {
        let outside = provenance
            .validity_of(name)
            .filter(|range| out_of_range(range, v));
        if let Some(range) = outside {
            warnings.push(format!(
                "{name} = {v} {unit} is outside the validated range {min} to {max} {unit} ({source})",
                unit = range.unit,
                min = range.min,
                max = range.max,
                source = provenance.source.label(),
            ));
        }
    }
    // A flag from the core with no matching input message still must not be
    // lost, so report it by name.
    if warnings.is_empty() && provenance.warnings.contains(Warnings::OUT_OF_RANGE) {
        warnings.push("OUT_OF_RANGE".to_owned());
    }
    if strict && !warnings.is_empty() {
        return Err(PyValueError::new_err(warnings.join("; ")));
    }
    let value = value.into_py_any(py)?;
    Ok(Tagged {
        value,
        source: provenance.source.label(),
        validity: provenance
            .validity
            .iter()
            .map(|v| (v.input, v.unit, v.min, v.max))
            .collect(),
        warnings,
    })
}
