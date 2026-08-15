# tpt-eng-geometry

Core 3D geometry primitives and operations for engineering applications.

Part of the [`tpt-engineering`](https://github.com/tpt-solutions/tpt-engineering) workspace. Dual-licensed
under MIT OR Apache-2.0.

## Features

- Points, vectors, frames, and affine transforms (built on [`glam`]).
- Curves: lines, circles, arcs, and Bézier curves (`Curve3` trait).
- Surfaces: planes, spheres, and cylinders (`Surface3` trait).
- Intersections: line/line, line/plane, line/sphere, plane/plane, ray/triangle.
- Projections: point-to-line and point-to-plane.
- Engineering queries: distances, angles, triangle area/perimeter, axis-aligned bounding boxes.

## Example

```rust
use tpt_eng_geometry::{Point3, curve::Line3, surface::Plane3, intersection};

let line = Line3::new(Point3::new(0.0, 0.0, -1.0), Point3::new(0.0, 0.0, 1.0));
let plane = Plane3::new(Point3::ZERO, Point3::new(0.0, 0.0, 1.0));
let (hit, _t) = intersection::line_plane(line, plane).unwrap();
assert!((hit - Point3::ZERO).length() < 1e-5);
```

[`glam`]: https://crates.io/crates/glam
