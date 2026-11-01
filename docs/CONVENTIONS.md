# Workspace conventions

These rules decide how the `tpt-eng-*` crates handle errors, units and numeric
precision. New code follows them. Existing exceptions are listed at the end and
are kept on purpose.

## 1. Error strategy

Pick the mechanism by what kind of failure it is:

| Situation | Mechanism | Example |
|---|---|---|
| Precondition on a physical input that has no meaning (non-positive dimension, zero base, NaN/inf where a finite value is required) | **Panic**, with a message and a `# Panics` section | `PerUnitSystem::new`, `nusselt_*` Re/Pr checks, `Beam::add` |
| Operation that can fail for data-dependent reasons (out-of-range property, solver not converging, input outside a supported region) | **`Result`** with a crate error enum, or **`Option`** when the only failure is "no answer" | `props-air::dew_point`, `props-water::state`, `Beam::try_add`, `impedance_parallel` |
| A documented domain where a non-finite value is the correct answer | Return it, and say so in the docs | `Complex` division by zero is `NaN`; use `impedance_parallel` for a short |

Rules:

- Do not return `NaN` silently for an input the caller could have checked. A
  `NaN` result is acceptable only when the function's docs name it.
- Every function that panics documents it under `# Panics`. Every `Result` or
  `Option` documents its failure cases under `# Errors`.
- A crate that already has a `try_*` form keeps `add`/`new` as the panicking
  convenience and points to the fallible form in its docs.

## 2. Unit convention

- **SI base units everywhere**: m, kg, s, K, Pa, N, W, V, A. Temperatures in
  kelvin unless the name says otherwise (for example `temp_c`).
- **Angles in radians** in new APIs.
- **Documented exceptions** (kept for compatibility, always named in the
  signature or docs):
  - `tpt-eng-geotech` and `tpt-eng-vehicle-dynamics`: angles in degrees, with
    `_deg` in the parameter names.
  - `tpt-eng-props-air::moist_air_enthalpy`: kJ per kg dry air (the psychrometric
    convention).
  - `tpt-eng-props-water` cp and enthalpy: kJ/(kg·K) and kJ/kg.
  - `tpt-eng-pcb`: trace geometry is in SI metres in the closed forms. Mil
    values appear only in the IPC area helpers, named `*_mil2`.

## 3. Numeric precision

- Analysis, property, and control crates use `f64`.
- Geometry, CAD, mesh, NURBS, and GD&T use `f32`, because they are
  graphics-scale and share one vector type across the stack.
- `tpt-eng-crystallography` mixes both. Its public API stays as it is, and new
  code should pick one by the rule above.

## 4. Known deviations (not changed)

- `tpt-eng-gdt::size_limits` accepts an `is_hole` flag that it ignores. It is
  kept for call-site clarity and documented as such.
