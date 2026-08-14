//! # tpt-eng — TPT Solutions engineering toolkit (facade)
//!
//! A single-name facade re-exporting the five `tpt-eng3` crates so downstream
//! users can depend on one crate instead of five. Dual-licensed under
//! `MIT OR Apache-2.0`.
//!
//! ```rust
//! use tpt_eng::{geometry::Point3, cad::{Sphere, Part}};
//!
//! let sphere = Sphere { center: Point3::ZERO, radius: 1.0 };
//! let part = Part::new("ball", Box::new(sphere));
//! let mesh = part.mesh(12);
//! assert!(mesh.face_count() > 0);
//! ```
//!
//! The re-exported crates are:
//!
//! - [`geometry`] — `tpt_eng_geometry`: primitives, transforms, curves, surfaces, intersections.
//! - [`mesh`] — `tpt_eng_mesh`: triangle mesh model, normals, quality, STL/OBJ I/O.
//! - [`nurbs`] — `tpt_eng_nurbs`: B-spline / NURBS curves and surfaces.
//! - [`gdt`] — `tpt_eng_gdt`: datums, tolerance frames/zones, fits, stack-up.
//! - [`cad`] — `tpt_eng_cad`: SDF solids, booleans, marching-tetrahedra, B-Rep.

pub use tpt_eng_cad as cad;
pub use tpt_eng_gdt as gdt;
pub use tpt_eng_geometry as geometry;
pub use tpt_eng_mesh as mesh;
pub use tpt_eng_nurbs as nurbs;

/// Commonly used vector/point aliases.
pub use tpt_eng_geometry::{Point3, Vector3};
