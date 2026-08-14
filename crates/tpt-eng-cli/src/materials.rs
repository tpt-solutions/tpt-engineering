//! Built-in material property table for the CLI `materials inspect` command.

/// A material with representative engineering properties.
#[derive(Debug, Clone)]
pub struct Material {
    /// Material name.
    pub name: &'static str,
    /// Density (kg/m^3).
    pub density: f64,
    /// Young's modulus (Pa).
    pub youngs_modulus: f64,
    /// Yield strength (Pa).
    pub yield_strength: f64,
    /// Poisson's ratio (dimensionless).
    pub poisson: f64,
}

/// Built-in materials.
pub const MATERIALS: &[Material] = &[
    Material { name: "steel", density: 7850.0, youngs_modulus: 200e9, yield_strength: 250e6, poisson: 0.30 },
    Material { name: "aluminium", density: 2700.0, youngs_modulus: 69e9, yield_strength: 95e6, poisson: 0.33 },
    Material { name: "concrete", density: 2400.0, youngs_modulus: 30e9, yield_strength: 40e6, poisson: 0.20 },
    Material { name: "timber", density: 500.0, youngs_modulus: 11e9, yield_strength: 30e6, poisson: 0.40 },
    Material { name: "titanium", density: 4500.0, youngs_modulus: 116e9, yield_strength: 880e6, poisson: 0.32 },
    Material { name: "copper", density: 8960.0, youngs_modulus: 117e9, yield_strength: 70e6, poisson: 0.34 },
];

/// Look up a material by name (case-insensitive). Returns `None` if not found.
pub fn find(name: &str) -> Option<&'static Material> {
    MATERIALS
        .iter()
        .find(|m| m.name.eq_ignore_ascii_case(name.trim()))
}

/// Format a material as a multi-line property listing.
pub fn describe(material: &Material) -> String {
    format!(
        "Material: {}\n  density:          {:.0} kg/m^3\n  Young's modulus:  {:.2e} Pa\n  yield strength:   {:.2e} Pa\n  Poisson's ratio:  {:.2}",
        material.name, material.density, material.youngs_modulus, material.yield_strength, material.poisson
    )
}
