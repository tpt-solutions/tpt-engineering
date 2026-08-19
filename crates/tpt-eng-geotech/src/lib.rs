//! # tpt-eng-geotech
//!
//! Soil mechanics primitives for the TPT engineering ecosystem: shear-strength
//! failure criteria, a reduced critical-state (Cam-Clay) model, and borehole
//! stratigraphy with provenance tracking.
//!
//! ## Units
//!
//! All stress, pressure, and strength quantities are in **pascals (Pa)** unless
//! noted otherwise, depths are in **metres (m)**, angles in **degrees**, and
//! void ratios are dimensionless. No unit library (e.g. `uom`) is used; SI units
//! are documented at each public item and are the caller's responsibility.
//!
//! ## Provenance
//!
//! Soil layers record where their data came from via the
//! [`tpt_eng_materials::DataSource`] type, shared with the materials crate so
//! that geotechnical records can participate in the same data-policy checks.
#![forbid(unsafe_code)]

use tpt_eng_materials::DataSource;

/// A single stratigraphic unit recovered or inferred from a borehole.
///
/// Depths are measured downwards from ground level (0.0 at the surface) in
/// metres. Strength parameters follow the Mohr-Coulomb model documented in the
/// [`mohr_coulomb`] module.
#[derive(Debug, Clone, PartialEq)]
pub struct SoilLayer {
    /// Top depth of the layer below ground level, in metres.
    pub depth_top: f64,
    /// Bottom depth of the layer below ground level, in metres.
    ///
    /// Must satisfy `depth_bottom > depth_top`; the crate does not enforce this
    /// but downstream summaries assume it.
    pub depth_bottom: f64,
    /// Free-text classification of the soil (e.g. `"sand"`, `"clay"`).
    pub soil_type: String,
    /// Mohr-Coulomb friction angle, in degrees.
    pub friction_angle_deg: f64,
    /// Mohr-Coulomb cohesion, in pascals.
    pub cohesion: f64,
    /// Bulk unit weight γ, in newtons per cubic metre (N/m³).
    pub unit_weight: f64,
    /// Provenance of this layer's data.
    pub source: DataSource,
}

impl SoilLayer {
    /// Thickness of the layer in metres (`depth_bottom - depth_top`).
    #[must_use]
    pub fn thickness(&self) -> f64 {
        self.depth_bottom - self.depth_top
    }
}

/// A vertical soil investigation (borehole) at a single location.
///
/// Layers are expected to be ordered from shallow to deep and (typically)
/// contiguous, but the summaries in this crate work with any ordering of
/// non-overlapping layers.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Borehole {
    /// Human-readable location label for the borehole.
    pub location: String,
    /// The stratigraphic layers, shallowest first.
    pub layers: Vec<SoilLayer>,
}

impl Borehole {
    /// Create an empty borehole at the given location.
    #[must_use]
    pub fn new(location: impl Into<String>) -> Self {
        Self {
            location: location.into(),
            layers: Vec::new(),
        }
    }

    /// Total investigated depth in metres, summed over all layer thicknesses.
    ///
    /// This is the cumulative thickness of the logged layers, not necessarily
    /// the depth of the deepest layer boundary if layers are non-contiguous.
    #[must_use]
    pub fn total_depth(&self) -> f64 {
        self.layers.iter().map(SoilLayer::thickness).sum()
    }

    /// Return the layer that contains depth `d` (metres below ground level).
    ///
    /// A layer contains `d` when `layer.depth_top <= d < layer.depth_bottom`.
    /// Returns `None` if `d` lies above the shallowest layer, below the deepest
    /// layer, or within a gap between non-contiguous layers.
    #[must_use]
    pub fn layer_at_depth(&self, d: f64) -> Option<&SoilLayer> {
        self.layers
            .iter()
            .find(|l| d >= l.depth_top && d < l.depth_bottom)
    }
}

/// Mohr-Coulomb and reduced critical-state soil models.
pub mod mohr_coulomb {
    /// Mohr-Coulomb shear strength (failure envelope): `τ_f = c + σ_n · tan φ`.
    ///
    /// # Parameters
    ///
    /// * `cohesion` — cohesion `c` in pascals.
    /// * `friction_angle_deg` — friction angle `φ` in degrees (`[0, 90)`).
    /// * `normal_stress` — total or effective normal stress `σ_n` in pascals.
    ///
    /// # Returns
    ///
    /// The shear strength `τ_f` in pascals at the given normal stress.
    #[must_use]
    pub fn shear_strength(cohesion: f64, friction_angle_deg: f64, normal_stress: f64) -> f64 {
        let phi = friction_angle_deg.to_radians();
        cohesion + normal_stress * phi.tan()
    }

    /// Factor of safety against Mohr-Coulomb shear failure.
    ///
    /// Defined as `FoS = τ_f / τ`, where `τ_f` is the [`shear_strength`] and
    /// `τ` is the applied shear stress. A value of `1.0` means the soil is
    /// exactly at failure; `FoS > 1` is stable and `FoS < 1` has failed.
    ///
    /// # Parameters
    ///
    /// * `shear_stress` — applied shear stress `τ` in pascals.
    /// * `cohesion` — cohesion `c` in pascals.
    /// * `friction_angle_deg` — friction angle `φ` in degrees.
    /// * `normal_stress` — normal stress `σ_n` in pascals.
    ///
    /// # Returns
    ///
    /// The dimensionless factor of safety. If `shear_stress` is `0.0` the result
    /// is `±∞` (or `NaN` for `0.0` strength), since this is ordinary `f64`
    /// division and no panic occurs.
    #[must_use]
    pub fn factor_of_safety(
        shear_stress: f64,
        cohesion: f64,
        friction_angle_deg: f64,
        normal_stress: f64,
    ) -> f64 {
        shear_strength(cohesion, friction_angle_deg, normal_stress) / shear_stress
    }
}

/// Reduced critical-state (Cam-Clay) soil model.
pub mod cam_clay {
    /// Evaluate the modified Cam-Clay yield surface.
    ///
    /// The surface is `f(p, q) = q² + M² · p · (p − p_c)`, where `p` is the mean
    /// effective stress, `q` is the deviatoric stress, `M` (`m`) is the slope of
    /// the critical-state line, and `p_c` is the pre-consolidation pressure.
    ///
    /// # Returns
    ///
    /// * `0.0` exactly on the yield surface,
    /// * `< 0.0` for an elastic (interior) state,
    /// * `> 0.0` for a state outside the yield surface (plastic over-stress).
    ///
    /// # Parameters
    ///
    /// * `p` — mean effective stress in pascals (should be `> 0`).
    /// * `q` — deviatoric stress in pascals.
    /// * `m` — critical-state line slope `M` (dimensionless, `> 0`).
    /// * `pc` — pre-consolidation pressure in pascals (`> 0`).
    #[must_use]
    pub fn cam_clay_yield(p: f64, q: f64, m: f64, pc: f64) -> f64 {
        q * q + m * m * p * (p - pc)
    }

    /// Whether a void-ratio change follows normal consolidation or swelling.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ConsolidationMode {
        /// Virgin loading: use the compression index `λ`.
        Normal,
        /// Unloading/reloading: use the swelling index `κ`.
        Swelling,
    }

    /// Update the void ratio `e` under a change in mean effective stress.
    ///
    /// For normal consolidation `e = e0 − λ · ln(p1 / p0)` and for swelling
    /// `e = e0 − κ · ln(p1 / p0)`, where `λ` (`lambda`) is the compression index
    /// and `κ` (`kappa`) is the swelling index.
    ///
    /// # Parameters
    ///
    /// * `e0` — initial void ratio (dimensionless).
    /// * `lambda` — compression index `λ` (dimensionless, normal consolidation).
    /// * `kappa` — swelling index `κ` (dimensionless, swelling).
    /// * `p0` — initial mean effective stress in pascals.
    /// * `p1` — final mean effective stress in pascals.
    /// * `mode` — [`ConsolidationMode`] selecting `λ` or `κ`.
    ///
    /// # Panics
    ///
    /// Panics if `p0 <= 0` or `p1 <= 0`, since the logarithm of a non-positive
    /// pressure is undefined.
    #[must_use]
    pub fn void_ratio_update(
        e0: f64,
        lambda: f64,
        kappa: f64,
        p0: f64,
        p1: f64,
        mode: ConsolidationMode,
    ) -> f64 {
        assert!(p0 > 0.0, "initial pressure p0 must be positive");
        assert!(p1 > 0.0, "final pressure p1 must be positive");
        let index = match mode {
            ConsolidationMode::Normal => lambda,
            ConsolidationMode::Swelling => kappa,
        };
        e0 - index * (p1 / p0).ln()
    }
}

pub use cam_clay::{ConsolidationMode, cam_clay_yield, void_ratio_update};
pub use mohr_coulomb::{factor_of_safety, shear_strength};
