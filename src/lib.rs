//! Geometric dimensioning and tolerancing (GD&T) data structures.
//!
//! This crate models the engineering concepts and metadata used to describe
//! geometric tolerances: material modifiers, geometric characteristics,
//! tolerance zones, datum reference frames, symbolic tolerance frames, fits and
//! allowances, and tolerance stack-up inputs. It is intentionally a data model
//! only and does not prescribe any particular inspection method.

use tpt_eng_geometry::frame::Frame3;
use tpt_eng_geometry::{Point3, EPSILON};

pub use tpt_eng_geometry::Point3 as WorldPoint;

/// A material condition modifier applied to a feature or datum reference.
///
/// Material modifiers describe how a tolerance or boundary behaves as the
/// actual size of a feature departs from a stated limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum MaterialCondition {
    /// No material condition is stated; the tolerance applies at any size.
    Rfs,
    /// Maximum Material Condition: the feature is at its largest (or smallest
    /// for an internal feature) allowable size.
    Mmc,
    /// Least Material Condition: the feature is at its smallest (or largest
    /// for an internal feature) allowable size.
    Lmc,
}

/// High-level grouping of geometric characteristics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ToleranceCategory {
    /// Form controls that constrain the shape of a feature.
    Form,
    /// Profile controls that constrain a line or surface outline.
    Profile,
    /// Orientation controls that constrain attitude relative to a datum.
    Orientation,
    /// Location controls that constrain position relative to a datum.
    Location,
    /// Runout controls that constrain the variation of a surface in rotation.
    Runout,
}

/// A geometric characteristic (the symbol/class of a geometric tolerance).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum GeometricCharacteristic {
    Flatness,
    Straightness,
    Circularity,
    Cylindricity,
    ProfileOfLine,
    ProfileOfSurface,
    Perpendicularity,
    Parallelism,
    Angularity,
    Position,
    Concentricity,
    Symmetry,
    CircularRunout,
    TotalRunout,
}

impl GeometricCharacteristic {
    /// Classify this characteristic into a [`ToleranceCategory`].
    #[must_use]
    pub fn category(&self) -> ToleranceCategory {
        match self {
            GeometricCharacteristic::Flatness
            | GeometricCharacteristic::Straightness
            | GeometricCharacteristic::Circularity
            | GeometricCharacteristic::Cylindricity => ToleranceCategory::Form,
            GeometricCharacteristic::ProfileOfLine | GeometricCharacteristic::ProfileOfSurface => {
                ToleranceCategory::Profile
            }
            GeometricCharacteristic::Perpendicularity
            | GeometricCharacteristic::Parallelism
            | GeometricCharacteristic::Angularity => ToleranceCategory::Orientation,
            GeometricCharacteristic::Position
            | GeometricCharacteristic::Concentricity
            | GeometricCharacteristic::Symmetry => ToleranceCategory::Location,
            GeometricCharacteristic::CircularRunout | GeometricCharacteristic::TotalRunout => {
                ToleranceCategory::Runout
            }
        }
    }
}

/// The shape and magnitude of a tolerance zone.
///
/// The magnitude is always expressed as a single scalar appropriate to the
/// zone shape (a diameter for round zones, a width for band-like zones).
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ToleranceZone {
    /// A cylindrical zone bounded by a diameter.
    Cylindrical { diameter: f32 },
    /// A zone bounded by two parallel planes separated by a width.
    ParallelPlanes { tolerance: f32 },
    /// A zone bounded by two parallel lines separated by a width.
    TwoParallelLines { tolerance: f32 },
    /// A spherical zone bounded by a diameter.
    Sphere { diameter: f32 },
    /// A circular zone (in a plane) bounded by a diameter.
    Circle { diameter: f32 },
    /// A band-shaped zone used for total runout.
    TotalRunoutBand { tolerance: f32 },
}

impl ToleranceZone {
    /// Return the governing magnitude of the zone.
    #[must_use]
    pub fn magnitude(&self) -> f32 {
        match self {
            ToleranceZone::Cylindrical { diameter } => *diameter,
            ToleranceZone::ParallelPlanes { tolerance } => *tolerance,
            ToleranceZone::TwoParallelLines { tolerance } => *tolerance,
            ToleranceZone::Sphere { diameter } => *diameter,
            ToleranceZone::Circle { diameter } => *diameter,
            ToleranceZone::TotalRunoutBand { tolerance } => *tolerance,
        }
    }
}

/// A datum: a referenced feature used as a basis for measurement, represented
/// by a named, oriented coordinate frame.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Datum {
    /// Single-letter label identifying the datum.
    pub label: char,
    /// The datum reference frame, expressed in world coordinates.
    pub frame: Frame3,
}

impl Datum {
    /// Construct a datum from a label and its reference frame.
    #[must_use]
    pub fn new(label: char, frame: Frame3) -> Self {
        Self { label, frame }
    }
}

/// A reference to a datum within a feature control frame, with an associated
/// material condition.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DatumReference {
    /// Single-letter label of the referenced datum.
    pub label: char,
    /// Material condition applied to the datum reference.
    pub condition: MaterialCondition,
}

impl DatumReference {
    /// Construct a datum reference from a label and condition.
    #[must_use]
    pub fn new(label: char, condition: MaterialCondition) -> Self {
        Self { label, condition }
    }
}

/// An ordered set of datums (primary, secondary, tertiary) forming a datum
/// reference frame. Secondary and tertiary datums are expressed relative to the
/// preceding datum.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DatumReferenceFrame {
    /// Primary datum, establishing the base frame.
    pub primary: Datum,
    /// Secondary datum, constraining one additional degree of freedom.
    pub secondary: Option<Datum>,
    /// Tertiary datum, constraining the final degree of freedom.
    pub tertiary: Option<Datum>,
}

impl DatumReferenceFrame {
    /// Construct a frame from a primary datum.
    #[must_use]
    pub fn new(primary: Datum) -> Self {
        Self {
            primary,
            secondary: None,
            tertiary: None,
        }
    }

    /// Add (or replace) the secondary datum and return the frame.
    #[must_use]
    pub fn with_secondary(mut self, datum: Datum) -> Self {
        self.secondary = Some(datum);
        self
    }

    /// Add (or replace) the tertiary datum and return the frame.
    #[must_use]
    pub fn with_tertiary(mut self, datum: Datum) -> Self {
        self.tertiary = Some(datum);
        self
    }

    /// Compose the datum frames into a single world frame.
    ///
    /// The secondary and tertiary datums are interpreted as being expressed
    /// relative to the preceding datum, so they are chained with [`Frame3::then`].
    #[must_use]
    pub fn world_frame(&self) -> Frame3 {
        let mut composed = self.primary.frame;
        if let Some(secondary) = &self.secondary {
            composed = composed.then(&secondary.frame);
        }
        if let Some(tertiary) = &self.tertiary {
            composed = composed.then(&tertiary.frame);
        }
        composed
    }

    /// Map a point given in the primary datum's local frame to world
    /// coordinates through the composed datum reference frame.
    #[must_use]
    pub fn to_world(&self, local: Point3) -> Point3 {
        self.world_frame().to_world_point(local)
    }
}

/// A symbolic geometric tolerance frame (feature control frame): a geometric
/// characteristic, a tolerance zone, and the datum references and modifiers
/// that apply to it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ToleranceFrame {
    /// The geometric characteristic being controlled.
    pub characteristic: GeometricCharacteristic,
    /// The tolerance zone shape and magnitude.
    pub zone: ToleranceZone,
    /// Ordered datum references applied by the frame.
    pub datum_refs: Vec<DatumReference>,
    /// Material condition modifiers carried by the frame.
    pub modifiers: Vec<MaterialCondition>,
}

impl ToleranceFrame {
    /// Construct a frame from a characteristic and zone, with no datum
    /// references or modifiers.
    #[must_use]
    pub fn new(characteristic: GeometricCharacteristic, zone: ToleranceZone) -> Self {
        Self {
            characteristic,
            zone,
            datum_refs: Vec::new(),
            modifiers: Vec::new(),
        }
    }

    /// Add a datum reference and return the frame (builder style).
    #[must_use]
    pub fn with_datum(mut self, reference: DatumReference) -> Self {
        self.datum_refs.push(reference);
        self
    }
}

/// A GD&T annotation attaching a tolerance frame to a named feature.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GdtAnnotation {
    /// Unique identifier for the annotation.
    pub id: String,
    /// The tolerance frame describing the requirement.
    pub frame: ToleranceFrame,
    /// Free-text description of the controlled feature.
    pub feature: String,
}

/// Classification of the functional relationship between a hole and a shaft.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum FitType {
    /// The shaft is always smaller than the hole (positive clearance).
    Clearance,
    /// The sizes overlap; assembly may need force or be loose depending on the
    /// actual parts.
    Transition,
    /// The shaft is always larger than the hole (negative clearance).
    Interference,
}

/// Upper and lower size limits for a feature.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Limits {
    /// Upper (maximum) limit.
    pub upper: f32,
    /// Lower (minimum) limit.
    pub lower: f32,
}

impl Limits {
    /// The nominal size, taken as the midpoint of the limits.
    #[must_use]
    pub fn nominal(&self) -> f32 {
        (self.upper + self.lower) / 2.0
    }

    /// The size tolerance, the difference between the limits.
    #[must_use]
    pub fn tolerance(&self) -> f32 {
        self.upper - self.lower
    }
}

/// Build symmetric size limits centered on `nominal`.
///
/// Both holes and shafts are treated identically here: the limits are
/// `(nominal - tolerance / 2, nominal + tolerance / 2)`. The `is_hole` flag is
/// retained for call-site clarity and possible future asymmetric conventions.
#[must_use]
pub fn size_limits(nominal: f32, tolerance: f32, is_hole: bool) -> Limits {
    let _ = is_hole;
    Limits {
        lower: nominal - tolerance / 2.0,
        upper: nominal + tolerance / 2.0,
    }
}

/// Minimum clearance between a hole and a shaft (hole lower limit minus shaft
/// upper limit). A negative result indicates interference.
#[must_use]
pub fn clearance(hole: &Limits, shaft: &Limits) -> f32 {
    hole.lower - shaft.upper
}

/// Classify the fit between a hole and a shaft using a small epsilon around
/// zero clearance.
#[must_use]
pub fn fit_type(hole: &Limits, shaft: &Limits) -> FitType {
    let c = clearance(hole, shaft);
    if c > EPSILON {
        FitType::Clearance
    } else if c < -EPSILON {
        FitType::Interference
    } else {
        FitType::Transition
    }
}

/// The intentional difference at maximum material, equal to the minimum
/// clearance between the hole and shaft.
#[must_use]
pub fn allowance(hole: &Limits, shaft: &Limits) -> f32 {
    clearance(hole, shaft)
}

/// A single contributor to a 1-D tolerance stack-up.
///
/// `sign` is `+1.0` or `-1.0` and indicates the direction in which the member
/// contributes to the overall stack. `tol_plus` and `tol_minus` are the
/// one-sided deviations in the member's own positive and negative directions.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StackupMember {
    /// Nominal contribution of the member.
    pub nominal: f32,
    /// Allowable positive deviation of the member.
    pub tol_plus: f32,
    /// Allowable negative deviation of the member.
    pub tol_minus: f32,
    /// Direction of contribution, `+1.0` or `-1.0`.
    pub sign: f64,
}

/// A one-dimensional tolerance stack-up composed of several members.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Stackup {
    /// The contributing members.
    pub members: Vec<StackupMember>,
}

impl Stackup {
    /// Construct a stack-up from its members.
    #[must_use]
    pub fn new(members: Vec<StackupMember>) -> Self {
        Self { members }
    }

    /// Nominal stack-up length: the signed sum of member nominal contributions.
    #[must_use]
    pub fn nominal(&self) -> f32 {
        self.members.iter().map(|m| m.sign as f32 * m.nominal).sum()
    }

    /// Worst-case bounds `(lower, upper)`.
    ///
    /// Each member contributes a one-sided range
    /// `[sign*nominal - (sign>0 ? tol_minus : tol_plus),
    ///    sign*nominal + (sign>0 ? tol_plus   : tol_minus)]`,
    /// and the bounds are the sums of those per-member ranges.
    #[must_use]
    pub fn worst_case(&self) -> (f32, f32) {
        let mut lo = 0.0_f32;
        let mut hi = 0.0_f32;
        for m in &self.members {
            let signed_nominal = m.sign as f32 * m.nominal;
            let (neg, pos) = if m.sign >= 0.0 {
                (m.tol_minus, m.tol_plus)
            } else {
                (m.tol_plus, m.tol_minus)
            };
            lo += signed_nominal - neg;
            hi += signed_nominal + pos;
        }
        (lo, hi)
    }

    /// Root-sum-square bounds `(lower, upper)`.
    ///
    /// Each member is reduced to an equivalent symmetric tolerance
    /// `t_i = sqrt((tol_plus^2 + tol_minus^2) / 2)`, and the band is the nominal
    /// plus/minus `sqrt(Σ t_i^2)`.
    #[must_use]
    pub fn rss(&self) -> (f32, f32) {
        let mut sum_sq = 0.0_f32;
        for m in &self.members {
            let t_i = ((m.tol_plus * m.tol_plus + m.tol_minus * m.tol_minus) / 2.0).sqrt();
            sum_sq += t_i * t_i;
        }
        let dev = sum_sq.sqrt();
        let n = self.nominal();
        (n - dev, n + dev)
    }
}

/// Convenience constant for a symmetric stack-up member with equal one-sided
/// tolerances.
impl StackupMember {
    /// Construct a symmetric member (equal `tol_plus` and `tol_minus`).
    #[must_use]
    pub fn symmetric(nominal: f32, tolerance: f32, sign: f64) -> Self {
        Self {
            nominal,
            tol_plus: tolerance,
            tol_minus: tolerance,
            sign,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::Quat;

    #[test]
    fn characteristic_categories() {
        assert_eq!(
            GeometricCharacteristic::Flatness.category(),
            ToleranceCategory::Form
        );
        assert_eq!(
            GeometricCharacteristic::Position.category(),
            ToleranceCategory::Location
        );
        assert_eq!(
            GeometricCharacteristic::TotalRunout.category(),
            ToleranceCategory::Runout
        );
    }

    #[test]
    fn zone_magnitude() {
        assert_eq!(
            ToleranceZone::Cylindrical { diameter: 0.1 }.magnitude(),
            0.1
        );
        assert_eq!(
            ToleranceZone::ParallelPlanes { tolerance: 0.2 }.magnitude(),
            0.2
        );
        assert_eq!(ToleranceZone::Circle { diameter: 0.3 }.magnitude(), 0.3);
    }

    #[test]
    fn datum_frame_identity_roundtrip() {
        let primary = Datum::new('A', Frame3::IDENTITY);
        let drf = DatumReferenceFrame::new(primary);
        assert!(
            (drf.to_world(Point3::new(1.0, 0.0, 0.0)) - Point3::new(1.0, 0.0, 0.0)).length() < 1e-6
        );
    }

    #[test]
    fn datum_frame_composition() {
        let primary = Datum::new('A', Frame3::from_origin(Point3::new(1.0, 0.0, 0.0)));
        let secondary = Datum::new('B', Frame3::from_origin(Point3::new(0.0, 2.0, 0.0)));
        let drf = DatumReferenceFrame::new(primary).with_secondary(secondary);
        // local (0,0,0) -> primary origin (1,0,0) -> + secondary offset (0,2,0)
        let world = drf.to_world(Point3::ZERO);
        assert!((world - Point3::new(1.0, 2.0, 0.0)).length() < 1e-6);

        // a rotated secondary reorients the composed frame
        let rot = Quat::from_rotation_z(std::f32::consts::FRAC_PI_2);
        let secondary_rot = Datum::new('B', Frame3::new(Point3::ZERO, rot));
        let drf2 = DatumReferenceFrame::new(primary).with_secondary(secondary_rot);
        let world_x = drf2.to_world(Point3::new(1.0, 0.0, 0.0));
        assert!((world_x - Point3::new(1.0, 1.0, 0.0)).length() < 1e-5);
    }

    #[test]
    fn fit_classification_clearance() {
        let hole = size_limits(10.0, 0.1, true);
        assert!((hole.lower - 9.95).abs() < 1e-6);
        assert!((hole.upper - 10.05).abs() < 1e-6);
        // A shaft clearly smaller than the hole yields positive clearance.
        let shaft = size_limits(9.8, 0.1, false);
        assert!((shaft.lower - 9.75).abs() < 1e-6);
        assert!((shaft.upper - 9.85).abs() < 1e-6);
        assert!(clearance(&hole, &shaft) > 0.0);
        assert_eq!(fit_type(&hole, &shaft), FitType::Clearance);
    }

    #[test]
    fn fit_classification_interference() {
        let hole = size_limits(10.0, 0.1, true);
        let shaft = size_limits(10.2, 0.1, false);
        assert!(clearance(&hole, &shaft) < 0.0);
        assert_eq!(fit_type(&hole, &shaft), FitType::Interference);
        assert!((allowance(&hole, &shaft) - clearance(&hole, &shaft)).abs() < 1e-6);
    }

    #[test]
    fn limits_accessors() {
        let l = Limits {
            upper: 10.05,
            lower: 9.95,
        };
        assert!((l.nominal() - 10.0).abs() < 1e-6);
        assert!((l.tolerance() - 0.1).abs() < 1e-6);
    }

    #[test]
    fn stackup_worst_case() {
        let members = vec![
            StackupMember::symmetric(1.0, 0.1, 1.0),
            StackupMember::symmetric(1.0, 0.1, 1.0),
        ];
        let s = Stackup::new(members);
        assert!((s.nominal() - 2.0).abs() < 1e-6);
        let (lo, hi) = s.worst_case();
        assert!((lo - 1.8).abs() < 1e-6);
        assert!((hi - 2.2).abs() < 1e-6);
    }

    #[test]
    fn stackup_rss() {
        let members = vec![
            StackupMember::symmetric(1.0, 0.1, 1.0),
            StackupMember::symmetric(1.0, 0.1, 1.0),
        ];
        let s = Stackup::new(members);
        let (lo, hi) = s.rss();
        // per-member t_i = 0.1; dev = sqrt(2 * 0.1^2) = sqrt(0.02) ≈ 0.1414
        let dev = (2.0_f32 * 0.1_f32.powi(2)).sqrt();
        assert!((s.nominal() - 2.0).abs() < 1e-6);
        assert!((hi - (2.0 + dev)).abs() < 1e-4);
        assert!((lo - (2.0 - dev)).abs() < 1e-4);
    }

    #[test]
    fn stackup_signed_members() {
        let members = vec![
            StackupMember::symmetric(1.0, 0.1, 1.0),
            StackupMember::symmetric(1.0, 0.1, -1.0),
        ];
        let s = Stackup::new(members);
        assert!((s.nominal() - 0.0).abs() < 1e-6);
        let (lo, hi) = s.worst_case();
        // member1: [0.9,1.1]; member2: [-1.1,-0.9]; sum [-0.2, 0.2]
        assert!((lo - (-0.2)).abs() < 1e-6);
        assert!((hi - 0.2).abs() < 1e-6);
    }

    #[test]
    fn tolerance_frame_builder() {
        let frame = ToleranceFrame::new(
            GeometricCharacteristic::Position,
            ToleranceZone::Cylindrical { diameter: 0.05 },
        )
        .with_datum(DatumReference::new('A', MaterialCondition::Mmc));
        assert_eq!(frame.datum_refs.len(), 1);
        assert_eq!(frame.characteristic.category(), ToleranceCategory::Location);
        assert_eq!(frame.zone.magnitude(), 0.05);
    }
}
