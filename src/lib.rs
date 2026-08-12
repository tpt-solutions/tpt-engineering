//! # tpt-eng-geometry
//!
//! Core 3D geometry primitives and operations for engineering applications.
//!
//! The crate provides a small, license-clean foundation (built on [`glam`]) covering:
//!
//! - **Points and vectors** — see [`point`] and [`vector`].
//! - **Frames and transforms** — local/world coordinate frames and affine transforms
//!   (see [`frame`] and [`transform`]).
//! - **Curves** — a [`curve::Curve3`] trait with line, circle, arc, and Bézier implementations.
//! - **Surfaces** — a [`surface::Surface3`] trait with plane, sphere, and cylinder implementations.
//! - **Intersections** — [`intersection`] module (line/line, line/plane, line/sphere, plane/plane,
//!   ray/triangle).
//! - **Projections** — [`projection`] module (point to line, point to plane).
//! - **Queries** — [`query`] module (distances, angles, triangle area, bounding boxes).
//!
//! `Point3` and `Vector3` are type aliases over [`glam::Vec3`]; use `Point3` where a position in
//! space is meant (subject to translation under affine maps) and `Vector3` where a free direction
//! or displacement is meant.

pub mod curve;
pub mod frame;
pub mod intersection;
pub mod point;
pub mod projection;
pub mod query;
pub mod surface;
pub mod transform;
pub mod vector;

pub use glam::{Mat3, Mat4, Quat};

/// A point in 3D space (a position; translated by affine maps).
pub type Point3 = glam::Vec3;

/// A free vector in 3D space (a direction/displacement; rotated but not translated by affine maps).
pub type Vector3 = glam::Vec3;

/// Machine epsilon used for geometric comparisons.
pub const EPSILON: f32 = 1e-6;
