//! Built-in cross-section catalogue and property calculations for the CLI `sections inspect`
//! command. Dimensions are in consistent length units; areas are that unit squared and second
//! moments of area are that unit to the fourth power.

/// A standard cross-section shape.
#[derive(Debug, Clone)]
pub enum Section {
    /// Rectangle of breadth `b` and height `h` (about centroidal axes).
    Rectangle { b: f64, h: f64 },
    /// Solid circle of diameter `d`.
    Circle { d: f64 },
    /// Symmetric I-beam: total depth `d`, flange width `bf`, flange thickness `tf`, web thickness `tw`.
    IBeam {
        d: f64,
        bf: f64,
        tf: f64,
        tw: f64,
    },
}

impl Section {
    /// Cross-sectional area.
    pub fn area(&self) -> f64 {
        match self {
            Section::Rectangle { b, h } => b * h,
            Section::Circle { d } => std::f64::consts::PI * d * d / 4.0,
            Section::IBeam { d, bf, tf, tw } => 2.0 * bf * tf + tw * (d - 2.0 * tf),
        }
    }

    /// Second moment of area about the strong (x) centroidal axis.
    pub fn second_moment_x(&self) -> f64 {
        match self {
            Section::Rectangle { b, h } => b * h.powi(3) / 12.0,
            Section::Circle { d } => std::f64::consts::PI * d.powi(4) / 64.0,
            Section::IBeam { d, bf, tf, tw } => {
                let h_web = d - 2.0 * tf;
                let i_flanges = 2.0 * (bf * tf.powi(3) / 12.0 + bf * tf * ((d - tf) / 2.0).powi(2));
                let i_web = tw * h_web.powi(3) / 12.0;
                i_flanges + i_web
            }
        }
    }

    /// Second moment of area about the weak (y) centroidal axis.
    pub fn second_moment_y(&self) -> f64 {
        match self {
            Section::Rectangle { b, h } => h * b.powi(3) / 12.0,
            Section::Circle { d } => std::f64::consts::PI * d.powi(4) / 64.0,
            Section::IBeam { d, bf, tf, tw } => {
                let i_flanges = 2.0 * (tf * bf.powi(3) / 12.0);
                let i_web = (d - 2.0 * tf) * tw.powi(3) / 12.0;
                i_flanges + i_web
            }
        }
    }

    /// Section modulus about the strong axis (`Ix / (h/2)`).
    pub fn section_modulus_x(&self) -> f64 {
        let h = match self {
            Section::Rectangle { h, .. } => *h,
            Section::Circle { d } => *d,
            Section::IBeam { d, .. } => *d,
        };
        self.second_moment_x() / (h / 2.0)
    }
}

/// Look up a named built-in section. Names: `rectangle`, `circle`, `ibeam` (with parameters supplied
/// by the caller through [`Section`] directly). This helper is provided for symmetry with the
/// materials module; section geometries are explicit.
pub fn describe(section: &Section) -> String {
    let kind = match section {
        Section::Rectangle { .. } => "Rectangle",
        Section::Circle { .. } => "Circle",
        Section::IBeam { .. } => "I-beam",
    };
    format!(
        "Section: {kind}\n  area:               {:.4e}\n  I_x (strong):        {:.4e}\n  I_y (weak):         {:.4e}\n  section modulus W_x: {:.4e}",
        section.area(),
        section.second_moment_x(),
        section.second_moment_y(),
        section.section_modulus_x(),
    )
}
