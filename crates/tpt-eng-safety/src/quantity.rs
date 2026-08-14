//! A minimal, dynamically-dimensioned quantity type local to this crate.
//!
//! `tpt-math-units` provides compile-time dimension-checked quantities, but
//! the limit-check API here is generic over *which* dimension is being
//! checked (any two design/allowable values, compared at runtime) — a shape
//! that needs a runtime-tagged dimension rather than a distinct Rust type per
//! unit. This is a small, self-contained wrapper for that purpose only.

use thiserror::Error;

/// The physical dimension a [`Quantity`] is expressed in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dimension {
    /// Pressure/stress (SI: pascal).
    Pressure,
    /// Length (SI: metre).
    Length,
}

/// A numeric value tagged with its physical dimension.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quantity {
    /// The numeric value, in the dimension's SI unit.
    pub value: f64,
    /// The physical dimension.
    pub dim: Dimension,
}

impl Quantity {
    /// Construct a quantity with an explicit dimension.
    pub fn new(value: f64, dim: Dimension) -> Self {
        Self { value, dim }
    }

    /// Construct a pressure/stress quantity, in pascals.
    pub fn pascals(value: f64) -> Self {
        Self::new(value, Dimension::Pressure)
    }

    /// Construct a length quantity, in metres.
    pub fn meters(value: f64) -> Self {
        Self::new(value, Dimension::Length)
    }
}

/// Error evaluating or combining [`Quantity`] values.
#[derive(Debug, Clone, Error)]
pub enum QuantityError {
    /// The two quantities involved have different dimensions.
    #[error("dimension mismatch: {0}")]
    DimensionMismatch(String),
}
