//! # tpt-eng-provenance
//!
//! Tags for engineering results: where a published correlation comes from,
//! the input ranges it is valid over, and any warnings raised by a call.
//!
//! A correlation evaluated outside its validity range still returns a number.
//! Without a tag, the caller cannot tell that the number is suspect. The
//! `*_tagged` functions in the property crates return a [`Tagged`] value
//! instead, with the same numerical result as the untagged function.
//!
//! This crate is `no_std` with no dependencies, so the property crates can use
//! it unconditionally.
//!
//! ## Example
//!
//! ```
//! use tpt_eng_provenance::{Source, Validity, Warnings, out_of_range};
//!
//! const TEMPERATURE: Validity = Validity {
//!     input: "temperature",
//!     unit: "K",
//!     min: 173.15,
//!     max: 473.15,
//! };
//!
//! assert!(!out_of_range(&TEMPERATURE, 300.0));
//! assert!(out_of_range(&TEMPERATURE, 600.0));
//! assert_eq!(Source::Standard("ASHRAE Fundamentals").label(), "ASHRAE Fundamentals");
//! assert!(Warnings::NONE.is_empty());
//! ```

#![no_std]

/// A value together with the provenance of the correlation that produced it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tagged<T> {
    /// The computed value, identical to the untagged function's result.
    pub value: T,
    /// Where the value came from and what it is valid for.
    pub provenance: Provenance,
}

impl<T> Tagged<T> {
    /// Pair a value with its provenance.
    pub fn new(value: T, provenance: Provenance) -> Self {
        Self { value, provenance }
    }
}

/// The most validity ranges one result can carry. The property functions use
/// one or two. A fixed size keeps this type `no_std` and allocation-free, and
/// lets a range be built at call time, such as a critical-temperature bound.
pub const MAX_VALIDITY: usize = 4;

/// Source, validity and warnings for one result.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Provenance {
    /// The published source of the correlation.
    pub source: Source,
    /// The validity range of each input the value depends on. Unused slots are `None`.
    pub validity: [Option<Validity>; MAX_VALIDITY],
    /// Warnings raised by this particular call.
    pub warnings: Warnings,
}

impl Provenance {
    /// Provenance with the given validity ranges and no warnings raised.
    ///
    /// # Panics
    ///
    /// Panics if more than [`MAX_VALIDITY`] ranges are given. That is a
    /// programming error: each function's ranges are fixed by its design.
    pub fn new(source: Source, ranges: &[Validity]) -> Self {
        assert!(
            ranges.len() <= MAX_VALIDITY,
            "a result can carry at most {MAX_VALIDITY} validity ranges"
        );
        let mut validity = [None; MAX_VALIDITY];
        for (slot, range) in validity.iter_mut().zip(ranges) {
            *slot = Some(*range);
        }
        Self {
            source,
            validity,
            warnings: Warnings::NONE,
        }
    }

    /// Provenance with the given warnings added.
    pub fn with_warnings(mut self, warnings: Warnings) -> Self {
        self.warnings.insert(warnings);
        self
    }

    /// The validity range recorded for the named input, if any.
    pub fn validity_of(&self, input: &str) -> Option<&Validity> {
        self.ranges().find(|v| v.input == input)
    }

    /// Every validity range recorded, in the order they were given.
    pub fn ranges(&self) -> impl Iterator<Item = &Validity> {
        self.validity.iter().flatten()
    }
}

/// The validity range of one input, in the input's own unit.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Validity {
    /// Input name, such as `"temperature"`.
    pub input: &'static str,
    /// Unit the bounds are expressed in, such as `"K"`.
    pub unit: &'static str,
    /// Lower bound, inclusive.
    pub min: f64,
    /// Upper bound, inclusive.
    pub max: f64,
}

/// Where a published value came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Source {
    /// A published standard or handbook, with its reference.
    Standard(&'static str),
    /// Open literature, with its reference.
    Literature(&'static str),
    /// Derived by a calculation in this workspace.
    Calculated,
    /// A manufacturer's data sheet, with its reference.
    Manufacturer(&'static str),
}

impl Source {
    /// A human-readable label for this source.
    pub fn label(&self) -> &'static str {
        match *self {
            Source::Standard(s) | Source::Literature(s) | Source::Manufacturer(s) => s,
            Source::Calculated => "calculated",
        }
    }
}

/// Bitset of warnings raised by one call.
///
/// A bitset rather than a `Vec`, so that it works without `alloc`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Warnings(u8);

impl Warnings {
    /// No warnings.
    pub const NONE: Warnings = Warnings(0);
    /// An input lies outside the validity range of the correlation.
    pub const OUT_OF_RANGE: Warnings = Warnings(1);
    /// The result is an extrapolation beyond the fitted data.
    pub const EXTRAPOLATED: Warnings = Warnings(2);
    /// The result comes from an approximate model, not a full correlation.
    pub const APPROXIMATE_MODEL: Warnings = Warnings(4);

    /// Whether no warnings are set.
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Whether every warning in `other` is set in `self`.
    pub const fn contains(self, other: Warnings) -> bool {
        self.0 & other.0 == other.0
    }

    /// Set every warning in `other`.
    pub fn insert(&mut self, other: Warnings) {
        self.0 |= other.0;
    }

    /// Names of the warnings that are set, in a fixed order.
    pub fn names(self) -> impl Iterator<Item = &'static str> {
        [
            (Warnings::OUT_OF_RANGE, "OUT_OF_RANGE"),
            (Warnings::EXTRAPOLATED, "EXTRAPOLATED"),
            (Warnings::APPROXIMATE_MODEL, "APPROXIMATE_MODEL"),
        ]
        .into_iter()
        .filter(move |(w, _)| self.contains(*w))
        .map(|(_, name)| name)
    }
}

/// Whether `value` lies outside the inclusive range of `validity`.
///
/// A non-finite value is treated as outside the range.
pub fn out_of_range(validity: &Validity, value: f64) -> bool {
    !(validity.min..=validity.max).contains(&value)
}

/// The [`Warnings::OUT_OF_RANGE`] flag if `value` is outside `validity`,
/// otherwise [`Warnings::NONE`].
pub fn range_warning(validity: &Validity, value: f64) -> Warnings {
    if out_of_range(validity, value) {
        Warnings::OUT_OF_RANGE
    } else {
        Warnings::NONE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const T: Validity = Validity {
        input: "temperature",
        unit: "K",
        min: 273.15,
        max: 373.15,
    };

    #[test]
    fn bounds_are_inclusive() {
        assert!(!out_of_range(&T, 273.15));
        assert!(!out_of_range(&T, 373.15));
        assert!(out_of_range(&T, 273.14));
        assert!(out_of_range(&T, 373.16));
    }

    #[test]
    fn non_finite_is_out_of_range() {
        assert!(out_of_range(&T, f64::NAN));
        assert!(out_of_range(&T, f64::INFINITY));
    }

    #[test]
    fn warnings_combine_and_name() {
        let mut w = Warnings::NONE;
        assert!(w.is_empty());
        w.insert(Warnings::OUT_OF_RANGE);
        w.insert(Warnings::APPROXIMATE_MODEL);
        assert!(w.contains(Warnings::OUT_OF_RANGE));
        assert!(!w.contains(Warnings::EXTRAPOLATED));
        let names: [&str; 2] = {
            let mut it = w.names();
            [it.next().unwrap(), it.next().unwrap()]
        };
        assert_eq!(names, ["OUT_OF_RANGE", "APPROXIMATE_MODEL"]);
    }

    #[test]
    fn provenance_looks_up_validity_by_name() {
        const RANGES: &[Validity] = &[T];
        let p = Provenance::new(Source::Calculated, RANGES);
        assert_eq!(p.validity_of("temperature"), Some(&T));
        assert_eq!(p.validity_of("pressure"), None);
        assert!(p.warnings.is_empty());
        assert!(
            p.with_warnings(Warnings::OUT_OF_RANGE)
                .warnings
                .contains(Warnings::OUT_OF_RANGE)
        );
    }
}
