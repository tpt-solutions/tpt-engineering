//! Fuel heating values, density, air–fuel ratio and CO₂ factors
//! (`tpt_eng.fuels`).

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use tpt_eng_props_fuels::{BlendedFuel as CoreBlendedFuel, Fuel as CoreFuel};
use tpt_math_units::uom::si::mass_density::kilogram_per_cubic_meter;

/// A fuel with literature-typical properties. Heating values are MJ/kg.
#[pyclass(module = "tpt_eng.fuels", name = "Fuel", eq, eq_int)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Fuel {
    Methane,
    Hydrogen,
    NaturalGas,
    Diesel,
}

impl Fuel {
    fn core(self) -> CoreFuel {
        match self {
            Fuel::Methane => CoreFuel::Methane,
            Fuel::Hydrogen => CoreFuel::Hydrogen,
            Fuel::NaturalGas => CoreFuel::NaturalGas,
            Fuel::Diesel => CoreFuel::Diesel,
        }
    }
}

#[pymethods]
impl Fuel {
    /// Lower heating value, MJ/kg.
    fn lhv_mj_kg(&self) -> f64 {
        self.core().lhv_mj_kg()
    }

    /// Higher heating value, MJ/kg.
    fn hhv_mj_kg(&self) -> f64 {
        self.core().hhv_mj_kg()
    }

    /// Density at reference conditions, kg/m³. Gases are at about 15 °C and
    /// 1 atm, and diesel is a liquid density.
    fn density_kg_m3(&self) -> f64 {
        self.core().density().get::<kilogram_per_cubic_meter>()
    }

    /// Stoichiometric air–fuel ratio, kg dry air per kg fuel.
    fn stoichiometric_air_fuel_ratio(&self) -> f64 {
        self.core().stoichiometric_air_fuel_ratio()
    }

    /// CO₂ emission factor, kg CO₂ per MJ of LHV.
    fn co2_kg_per_mj(&self) -> f64 {
        self.core().co2_kg_per_mj()
    }
}

/// A natural gas and hydrogen blend. `h2_fraction` is the hydrogen mole
/// fraction, from 0 (pure natural gas) to 1 (pure hydrogen).
#[pyclass(module = "tpt_eng.fuels", name = "BlendedFuel", frozen)]
pub struct BlendedFuel {
    inner: CoreBlendedFuel,
}

#[pymethods]
impl BlendedFuel {
    #[new]
    fn new(h2_fraction: f64) -> PyResult<Self> {
        if !(0.0..=1.0).contains(&h2_fraction) {
            return Err(PyValueError::new_err(format!(
                "h2_fraction must be between 0 and 1, got {h2_fraction}"
            )));
        }
        Ok(Self {
            inner: CoreBlendedFuel::new(h2_fraction),
        })
    }

    /// Hydrogen mole fraction.
    #[getter]
    fn h2_fraction(&self) -> f64 {
        self.inner.h2_fraction
    }

    /// Effective molar mass, kg/kmol.
    #[getter]
    fn molar_mass_kg_per_kmol(&self) -> f64 {
        self.inner.molar_mass
    }

    /// Lower heating value, MJ/kg.
    #[getter]
    fn lhv_mj_kg(&self) -> f64 {
        self.inner.lhv
    }

    /// Higher heating value, MJ/kg.
    #[getter]
    fn hhv_mj_kg(&self) -> f64 {
        self.inner.hhv
    }

    /// Density at reference conditions, kg/m³.
    #[getter]
    fn density_kg_m3(&self) -> f64 {
        self.inner.density.get::<kilogram_per_cubic_meter>()
    }

    /// Stoichiometric air–fuel ratio, kg dry air per kg fuel.
    fn stoichiometric_air_fuel_ratio(&self) -> f64 {
        self.inner.stoichiometric_air_fuel_ratio()
    }

    /// CO₂ emission factor, kg CO₂ per MJ of LHV. Hydrogen adds none.
    fn co2_kg_per_mj(&self) -> f64 {
        self.inner.co2_kg_per_mj()
    }
}

/// Register the fuel bindings on the `tpt_eng.fuels` submodule.
pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Fuel>()?;
    m.add_class::<BlendedFuel>()?;
    Ok(())
}
