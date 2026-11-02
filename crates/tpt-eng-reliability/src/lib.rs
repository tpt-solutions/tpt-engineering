//! Reliability and life analysis.
//!
//! Covers fatigue (S–N and Miner's rule), life distributions (Weibull and
//! exponential), failure-rate helpers, FMEA structures, and probabilistic
//! design (reliability index / normal-integration helpers).
//!
//! ## Example
//!
//! ```
//! use tpt_eng_reliability::miners_rule;
//!
//! // Two stress blocks: 1 000 of 10 000 cycles and 5 000 of 50 000 cycles.
//! let damage = miners_rule(&[(1_000.0, 10_000.0), (5_000.0, 50_000.0)]);
//! assert!((damage - 0.2).abs() < 1e-12);
//! // Failure is reached when cumulative damage is at least 1.0.
//! assert!(miners_rule(&[(10.0, 10.0)]) >= 1.0);
//! ```

pub mod fatigue;
pub mod fmea;
pub mod life;
pub mod probabilistic;

pub use fatigue::*;
pub use fmea::*;
pub use life::*;
pub use probabilistic::*;

use thiserror::Error;

/// Errors produced by reliability routines.
#[derive(Debug, Clone, PartialEq, Error)]
pub enum ReliabilityError {
    /// An input parameter was outside its valid domain.
    #[error("invalid parameter: {0}")]
    InvalidParameter(String),
}
