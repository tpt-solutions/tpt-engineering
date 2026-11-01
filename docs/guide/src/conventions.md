# Units, errors and precision

The full rules are in
[`docs/CONVENTIONS.md`](https://github.com/tpt-solutions/tpt-engineering/blob/master/docs/CONVENTIONS.md).
In short:

## Units

- SI base units throughout: m, kg, s, K, Pa, N, W, V, A.
- Angles are radians in new APIs.
- Documented exceptions: degrees in `geotech` and `vehicle-dynamics` (`_deg`
  parameters), kJ in moist-air enthalpy, and kJ/(kg·K) for water cp.

## Errors

| Failure | Mechanism |
|---|---|
| Physically meaningless input (zero base, non-positive dimension) | Panic, documented under `# Panics` |
| Data-dependent failure (out-of-range property, no convergence) | `Result` with the crate's error type, or `Option` when the only failure is "no answer" |

## Precision

- Analysis, property and control crates use `f64`.
- Geometry, CAD, mesh, NURBS and GD&T use `f32`.
