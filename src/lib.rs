//! # tpt-eng-materials
//!
//! Material property modeling for the TPT engineering ecosystem.
//!
//! This crate models materials as a named, categorized collection of
//! [`material::Material`] values, each carrying typed [`property::Property`]
//! data and provenance. Three property forms are supported:
//!
//! * **scalar** — a single value with a unit string,
//! * **temperature-dependent** — `(temperature, value)` samples evaluated by
//!   linear interpolation ([`property::Property::value_at`]), and
//! * **anisotropic** — per-direction scalars (a hook for directional or
//!   tensor-valued properties).
//!
//! ## Data policy
//!
//! **Only original, user-provided, or openly licensed data is accepted.** Every
//! material must record a source ([`tpt_eng_core::DataSource`]), and
//! [`library::MaterialLibrary::validate`] enforces that rule: a material with no
//! recorded source, or a disallowed `license` attribute, fails validation. No
//! proprietary material tables are bundled or scraped.
//!
//! ## Persistence
//!
//! Libraries load/save as JSON ([`library::MaterialLibrary::from_json`] /
//! [`library::MaterialLibrary::to_json`]) and as CSV
//! ([`library::MaterialLibrary::from_csv`] / [`library::MaterialLibrary::to_csv`]).
//!
//! An optional embedded-database backend was evaluated and deliberately **not**
//! adopted: the in-memory + JSON/CSV model keeps the dependency tree small and
//! fully license-clean (all of `serde`, `serde_json`, and `csv` are
//! MIT/Apache-2.0 compatible).

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod category;
pub mod error;
pub mod library;
pub mod material;
pub mod property;

pub use category::MaterialCategory;
pub use error::{MaterialError, Result};
pub use library::{MaterialLibrary, ALLOWED_LICENSES};
pub use material::Material;
pub use property::{Property, TempPoint};

/// The most commonly used items, in one `use`.
pub mod prelude {
    pub use crate::{
        category::MaterialCategory,
        error::{MaterialError, Result},
        library::{MaterialLibrary, ALLOWED_LICENSES},
        material::Material,
        property::{Property, TempPoint},
    };
}
