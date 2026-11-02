---
name: Feature request
about: A new calculation, correlation, or unit/convention change
labels: enhancement
---

**Domain and crate**
Which `tpt-eng-*` crate this belongs in, or whether it needs a new crate
(see `docs/CONVENTIONS.md` for scope and error-handling rules).

**Method and source**
The equation or standard to implement, with a reference a reviewer can check.
Explain the validity range, since out-of-range use should be flagged, not silent.

**Units**
The inputs and outputs in SI, or the documented exception that applies.

**Validation**
A reference point to check the implementation against (see `docs/VALIDATION.md`).
