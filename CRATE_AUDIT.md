# Workspace Crate Audit: Existing Equivalents in the Rust Ecosystem

Generated 2026-08-15. Answers the question: for each crate in this workspace, is there
already a well-maintained Rust crate that does the same thing?

## Summary

Of the 43 workspace crates, the large majority are genuine, purpose-built engineering-domain
logic with **no direct Rust-ecosystem equivalent** — this is a niche domain (GD&T, tolerance
stack-up, IAPWS steam tables, cross-section properties, reliability/FMEA, structural beam
statics) that Rust simply doesn't have libraries for. A handful of crates do overlap
general-purpose crates, and in most of those cases the workspace's own doc comments say the
overlap is *deliberate* (explicit "license-clean" / "no external geometry-kernel deps"
language in `tpt-eng-cad` and `tpt-eng-mesh`), not an oversight.

## Crates with a real well-maintained equivalent worth a look

| Crate | Overlaps with | Notes |
|---|---|---|
| `tpt-eng-geo-topology` | `petgraph` | Custom `HashMap`/`HashSet` adjacency + hand-rolled BFS for upstream/downstream. `petgraph` is the de facto standard (10k+ stars), has Dijkstra/topo-sort/cycle-detection for free. Domain concepts (`EdgeKind`, `capacity`) would remain a thin wrapper either way. |
| `tpt-eng-geo-asset` | `geo` (georust) | Custom haversine implementation + `HashMap` registry. `geo`/`geo-types` provide `Point`, haversine/geodesic distance, and are the standard georust primitives. Overlap is partial — the asset-registry part is still domain-specific. |
| `tpt-eng-nurbs` | `truck` (`truck-geometry`) | Custom knot vectors + Cox–de Boor/de Boor implementation. `truck` has a maintained NURBS module, but it's a full CAD-kernel dependency (heavy) — pulling it in just for curve math may not be worth the coupling. |
| `tpt-eng-mesh` / `tpt-eng-io` | `tobj`, `obj-rs`, `stl_io` | Custom STL/OBJ parsers. These are well-maintained, widely used crates for exactly this. **However**, both crates' doc comments explicitly say "license-clean" — this looks like a deliberate choice (avoiding a dependency's license or transitive deps), not an unawareness gap. Worth confirming intent before touching. |
| `tpt-eng-cad` | `fidget`, `csgrs` | Custom SDF/CSG kernel + marching-tetrahedra. Same "license-clean, no external geometry-kernel deps" rationale as above — likely intentional. |

## Crates that are genuinely novel (no meaningful Rust equivalent)

`tpt-eng-cli`, `tpt-eng-controls` (no comprehensive Rust control-systems crate exists; only
single-purpose `pid`), `tpt-eng-gdt`, `tpt-eng-geometry` (thin `glam` wrapper + domain frames,
not a reimplementation), `tpt-eng-materials`, `tpt-eng-network-matrix`, `tpt-eng-plot` (already
built *on* `plotters`, not reinventing it), `tpt-eng-props` / `-air` / `-fuels` / `-water`
(ASHRAE/IAPWS-IF97 formulas — no maintained Rust equivalent), `tpt-eng-reliability`,
`tpt-eng-report`, `tpt-eng-safety`, `tpt-eng-sections`, `tpt-eng-standards`,
`tpt-eng-structural`, `tpt-eng-timeseries` family, `tpt-eng-tolerance` (already reuses
`rand`/`rand_distr` for Monte Carlo — appropriate reuse, not reinvention).

## Full per-crate inventory

| Crate | Purpose | LOC (src/*.rs) | Core technique |
|---|---|---|---|
| **tpt-eng-cad** | License-clean CAD/solid-modeling kernel using signed-distance fields (SDF): primitives, boolean CSG, mesh extraction, minimal B-Rep, feature modeling | 905 | In-house SDF solids + marching-tetrahedra isosurface extraction; no external geometry-kernel deps |
| **tpt-eng-cli** | Command-line interface tying the TPT engineering ecosystem together (controls, materials, props, sections, tolerance, units subcommands) | 1360 | `clap`-based CLI dispatching to the other crates; thin wrapper, no novel algorithms |
| **tpt-eng-controls** | Control-systems primitives: discrete-time PID controller, transfer-function model, state-space model (forward-Euler) | 359 | Wraps `tpt_math_linalg` dense matrices for state-space; from-scratch textbook formulations |
| **tpt-eng-examples** | Example programs (`mechanical_design`, `thermal_loop`) demonstrating cross-crate usage | 421 | Example/demo binaries, no reusable library logic |
| **tpt-eng-gdt** | GD&T (geometric dimensioning & tolerancing) data model: material modifiers, tolerance zones, datum reference frames, symbolic tolerance frames, fits/allowances, stack-up inputs | 772 | Pure data-model enums/structs over `tpt_eng_geometry` frames; no inspection logic |
| **tpt-eng-geo-asset** | Geographic asset registry mapping coordinates to logical device/network nodes | 215 | `HashMap`-based registry + haversine great-circle distance for spatial queries |
| **tpt-eng-geo-topology** | Directional infrastructure graphs (pipes/wires/ducts) over geo-asset node identities | 224 | Directed graph (adjacency via `HashMap`/`HashSet`) with BFS-style upstream/downstream traversal |
| **tpt-eng-geometry** | Core 3D geometry primitives: points/vectors, frames/transforms, curves, surfaces, intersections, projections, queries | 1194 | Thin wrapper over `glam::Vec3` with trait-based curve/surface abstractions |
| **tpt-eng-io** | Engineering file I/O: JSON, CSV, STL, OBJ read/write | 585 | Trait-based (de)serialization (`ReadFromFile`/`WriteToFile`/`EngineeringData`) over `serde`; STL/OBJ exchanged as `tpt_eng_mesh::Mesh` |
| **tpt-eng-materials** | Material property modeling: named/categorized materials with scalar, temperature-dependent, and anisotropic properties, with provenance/licensing validation | 1115 | Domain data model + linear interpolation for temp-dependent properties; JSON/CSV persistence, no embedded DB |
| **tpt-eng-mesh** | License-clean indexed triangle-mesh crate for CAD workloads: normals, quality metrics, refinement/repair, STL/OBJ conversion | 893 | Custom indexed mesh struct (positions/indices/normals); in-house STL/OBJ parsers, area-weighted normal computation |
| **tpt-eng-network-matrix** | Generates network matrices (incidence, admittance/Laplacian) from an infrastructure topology for solver consumption | 143 | Builds `DMatrix` (from `tpt_math_linalg`) via `A · diag(y) · Aᵀ` construction |
| **tpt-eng-nurbs** | In-house B-spline and NURBS curve/surface modeling | 727 | Custom knot vectors + Cox–de Boor basis functions and de Boor algorithm; no external NURBS lib |
| **tpt-eng-plot** | Plotting/diagram generation: XY charts, result bar charts, section drawings, PNG/SVG export | 1325 | Built on `plotters` crate; `Drawing` trait unifying chart types |
| **tpt-eng-props** | Umbrella crate re-exporting the fluid-property sub-crates (water/air/fuels) | 15 | Pure re-export module, `no_std` |
| **tpt-eng-props-air** | ASHRAE moist-air psychrometric properties for HVAC/combustion | 267 | Hyland–Wexler saturation-pressure correlation; `uom`-typed quantities, `no_std`-capable |
| **tpt-eng-props-fuels** | Fuel properties: heating values, density, stoichiometric air-fuel ratio, CO₂ emission factors | 244 | Lookup-style per-fuel constants/formulas (`Fuel` enum methods), `no_std`-capable |
| **tpt-eng-props-water** | IAPWS-IF97 water/steam property tables (regions 1, 2, 4) | 569 | Implements IAPWS-IF97 Gibbs free-energy fundamental equations directly; `uom`-typed, `no_std`-capable |
| **tpt-eng-reliability** | Reliability/life analysis: fatigue (S-N, Miner's rule), Weibull/exponential life distributions, failure rates, FMEA, probabilistic design | 494 | Domain formulas across `fatigue`/`fmea`/`life`/`probabilistic` submodules; `thiserror`-based error type |
| **tpt-eng-report** | Calculation-report data model with Markdown/HTML/JSON exporters and validation | 766 | Serializable `Report` model (`model`) + separate renderer modules per format |
| **tpt-eng-safety** | Safety margins and limit-state evaluation (utilization, margin, safety factor, pass/warn/fail) | 363 | Simple formula-based checks (`limit`/`quantity` submodules) producing structured `CheckReport` |
| **tpt-eng-sections** | Cross-section properties (area, centroid, moments, section/plastic moduli, torsion constant) for standard and custom shapes | 1122 | `Section` trait; composite shapes via rectangle decomposition, custom polygons via exact Green's-theorem formulas + grid-based plastic/torsion calc |
| **tpt-eng-standards** | Structured, parameterized modeling of standards-based load cases/combinations/factors/limit-state checks (no copyrighted standard text) | 694 | Generic data structures (`load`, `combinations`, `factors`, `limit_states`, `design`) evaluated against user-supplied demand maps |
| **tpt-eng-structural** | Structural engineering primitives: load definitions, simply-supported beam analysis, demand/capacity code-compliance checks | 374 | Closed-form beam statics (reactions/shear/bending moment); `uom`-typed quantities, no matrix solver needed |
| **tpt-eng-timeseries** | Umbrella crate re-exporting the timeseries-core/align/gap sub-crates | 12 | Pure re-export module |
| **tpt-eng-timeseries-align** | Aligns irregular multi-rate sensor streams onto a deterministic time grid | 145 | Clamped linear interpolation resampling onto a target grid vector |
| **tpt-eng-timeseries-core** | Core time-series types (`Timestamp`, `Sample<T>`, `Series`) shared across the timeseries family | 158 | Minimal `std`-only newtype/struct wrappers |
| **tpt-eng-timeseries-gap** | Staleness and gap detection/repair for dropout-prone sensor streams | 195 | Gap detection via max-`dt` threshold scan; hold/linear/zero-order fill `Strategy` enum |
| **tpt-eng-tolerance** | Tolerance stack-up analysis for mechanical dimensioning | 464 | Worst-case, RSS, and Monte Carlo methods (`rand`/`rand_distr`) with sensitivity/contributor ranking |
| **tpt-eng-electrical** | Electrical primitives: impedance/reactance, per-unit systems, three-phase power, conductor/insulator property lookups | stub | Scaffolded in Phase 9 (spec2.txt); implementation deferred |
| **tpt-eng-heat-transfer** | Heat-transfer correlations: convection (Nu/Re/Pr), 1D/radial conduction, radiation view factors, thermal-resistance networks | stub | Scaffolded in Phase 9; implementation deferred |
| **tpt-eng-props-mixture** | General real-gas / VLE property lookups for arbitrary process mixtures (`no_std`); joins the `tpt-eng-props` umbrella | stub | Scaffolded in Phase 9; re-exported by `tpt-eng-props` |
| **tpt-eng-pcb** | PCB layer stackup, trace routing primitives, via/footprint definitions | stub | Scaffolded in Phase 9; depends on `tpt-eng-electrical`, `tpt-eng-materials` |
| **tpt-eng-thermal-mgmt** | Heat sink sizing, fan curves, thermal-resistance networks for electronics | stub | Scaffolded in Phase 9; depends on `tpt-eng-heat-transfer`, `tpt-eng-props-air` |
| **tpt-eng-power-components** | Transformer/generator equivalent circuits, transmission-line parameters | stub | Scaffolded in Phase 9; depends on `tpt-eng-electrical` |
| **tpt-eng-renewables** | Solar PV, wind turbine, battery degradation models | stub | Scaffolded in Phase 9; depends on `tpt-eng-electrical`, `tpt-eng-props-air`, `tpt-eng-reliability` |
| **tpt-eng-vehicle-dynamics** | Tire models, aero drag, suspension kinematics | stub | Scaffolded in Phase 9; depends on `tpt-eng-geometry`, `tpt-eng-structural`, `tpt-math-linalg` |
| **tpt-eng-biomech** | Hyperelastic tissue models, implant geometry | stub | Scaffolded in Phase 9; depends on `tpt-eng-materials`, `tpt-eng-geometry` |
| **tpt-eng-unit-ops** | Distillation, heat exchangers, pump/compressor curves | stub | Scaffolded in Phase 9; depends on `tpt-eng-props`, `tpt-eng-heat-transfer` |
| **tpt-eng-crystallography** | Miller indices, slip systems, crystal symmetry | stub | Scaffolded in Phase 9; depends on `tpt-eng-geometry` |
| **tpt-eng-geotech** | Soil constitutive models, borehole stratigraphy | stub | Scaffolded in Phase 9; depends on `tpt-eng-materials`, `tpt-math-linalg` |
| **tpt-eng-building-sys** | HVAC loads, plumbing fixture units, electrical panel scheduling | stub | Scaffolded in Phase 9; depends on `tpt-eng-props-air`, `tpt-eng-heat-transfer`, `tpt-eng-electrical` |
| **tpt-eng-schedule** | CPM/PERT networks, resource leveling, Earned Value Management | stub | Scaffolded in Phase 9; depends only on `tpt-math-numeric` |

### Notable architectural patterns across the workspace

- **Umbrella crates** (`tpt-eng-props`, `tpt-eng-timeseries`) are near-empty re-export shims (12–15 LOC) grouping a family of sibling crates.
- **License-clean, in-house implementations** are a recurring theme — CAD/SDF (`tpt-eng-cad`), mesh I/O (`tpt-eng-mesh`), NURBS (`tpt-eng-nurbs`) all explicitly avoid third-party geometry-kernel/format crates.
- **`uom`-typed units** (via `tpt_math_units`) are used consistently in the physical-property/engineering-analysis crates (`props-air`, `props-fuels`, `props-water`, `structural`), while pure-geometry crates (`geometry`, `mesh`, `cad`, `nurbs`) use plain `f32`/`glam`.
- **Domain-data-only crates** with explicit "no proprietary data/text" legal disclaimers: `tpt-eng-materials`, `tpt-eng-standards`, `tpt-eng-gdt`.
- The largest crates by LOC are `tpt-eng-plot` (1325), `tpt-eng-geometry` (1194), `tpt-eng-sections` (1122), and `tpt-eng-materials` (1115); the smallest are the two umbrella crates and `tpt-eng-network-matrix` (143).
