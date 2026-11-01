//! Python bindings for tpt-engineering (PyO3).
//!
//! The top-level `tpt_eng` module holds IAPWS-IF97 water and steam properties.
//! `tpt_eng.controls` and `tpt_eng.structural` are submodules.
//!
//! Conventions for every binding:
//!
//! * Inputs and outputs are plain `float`. SI units are part of each name
//!   (`_k`, `_pa`, `_nm`, `_s`), so Python callers never guess units.
//! * Core errors become Python exceptions. Out-of-range inputs raise
//!   `ValueError`, and unimplemented regions raise `NotImplementedError`.
//! * Panics in the core are avoided by validating inputs here first, so Python
//!   never sees a `PanicException`.

mod controls;
mod structural;
mod water;

use pyo3::prelude::*;
use pyo3::types::PyModule;

/// The public package name. maturin wraps this native module in a `tpt_eng`
/// package, so the native module's own `__name__` is `tpt_eng.tpt_eng`. Build
/// submodule names from this constant rather than from `__name__`.
const PACKAGE: &str = "tpt_eng";

/// Create a submodule, attach it to `parent`, and register it in
/// `sys.modules` so `import tpt_eng.<name>` works.
fn add_submodule(
    py: Python<'_>,
    parent: &Bound<'_, PyModule>,
    name: &str,
    register: fn(&Bound<'_, PyModule>) -> PyResult<()>,
) -> PyResult<()> {
    let sub = PyModule::new(py, name)?;
    register(&sub)?;
    parent.add_submodule(&sub)?;
    py.import("sys")?
        .getattr("modules")?
        .set_item(format!("{PACKAGE}.{name}"), &sub)?;
    Ok(())
}

#[pymodule]
fn tpt_eng(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    water::register(m)?;
    add_submodule(py, m, "controls", controls::register)?;
    add_submodule(py, m, "structural", structural::register)?;
    Ok(())
}
