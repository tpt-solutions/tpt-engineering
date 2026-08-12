//! # tpt-eng-standards
//!
//! Standards modeling as **structured, parameterized data** for the TPT
//! engineering ecosystem.
//!
//! > **Legal rule.** This crate contains **no** copyrighted standard text, no
//! > proprietary code clauses, and no scraped proprietary tables. It models the
//! > *shape* of standards-based calculations — load cases, load combinations,
//! > load/safety factors, and limit-state logic — as generic, user-filled data
//! > structures. The actual factors and combinations are supplied by the user
//! > (who is responsible for the license of the values they enter).
//!
//! ## Modules
//!
//! * [`load`] — [`load::LoadCase`] and [`load::LoadType`].
//! * [`combinations`] — [`combinations::LoadCombination`] and the arithmetic to
//!   evaluate one against a demand map. Combinations and their factors are
//!   user-provided data.
//! * [`factors`] — [`factors::FactorSet`], a user-supplied bag of named partial
//!   factors.
//! * [`limit_states`] — [`limit_states::LimitState`] and the parameterized
//!   [`limit_states::DemandCapacity`] utilization check.
//! * [`design`] — the [`design::DesignBasis`] aggregate and
//!   [`design::evaluate_check`]/[`design::CheckResult`] workflow that ties a
//!   combination to a limit state and a capacity.
//!
//! ## User-provided data
//!
//! Every factor, combination, and factor set is plain data the caller provides.
//! See the module examples for building a [`design::DesignBasis`] from your own
//! (correctly licensed) values.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod combinations;
pub mod design;
pub mod factors;
pub mod limit_states;
pub mod load;

pub use combinations::{CombinationFactor, LoadCombination};
pub use design::{evaluate_check, CheckResult, DesignBasis};
pub use factors::{FactorSet, LoadFactor};
pub use limit_states::{DemandCapacity, LimitState};
pub use load::{LoadCase, LoadType};

/// The most commonly used items, in one `use`.
pub mod prelude {
    pub use crate::{
        combinations::{CombinationFactor, LoadCombination},
        design::{evaluate_check, CheckResult, DesignBasis},
        factors::{FactorSet, LoadFactor},
        limit_states::{DemandCapacity, LimitState},
        load::{LoadCase, LoadType},
    };
}
