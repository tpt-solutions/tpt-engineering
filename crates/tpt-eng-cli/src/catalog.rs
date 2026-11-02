//! Catalog of the publishable `tpt-eng-*` crates, used by `tpt-eng crates`
//! and `tpt-eng recommend`. A unit test keeps it in step with the workspace.

/// One crate in the catalog.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Entry {
    /// Crate name as published on crates.io.
    pub name: &'static str,
    /// One-line summary.
    pub summary: &'static str,
    /// Search keywords.
    pub keywords: &'static [&'static str],
}

/// Every publishable workspace crate.
pub const CATALOG: &[Entry] = &[
    Entry {
        name: "tpt-eng-biomech",
        summary: "Hyperelastic tissue constitutive models, implant geometry.",
        keywords: &[
            "engineering",
            "biomechanics",
            "hyperelastic",
            "medical",
            "implant",
        ],
    },
    Entry {
        name: "tpt-eng-building-sys",
        summary: "HVAC load calculation, plumbing fixture units, electrical panel scheduling.",
        keywords: &[
            "engineering",
            "hvac",
            "building",
            "plumbing",
            "electrical-panel",
        ],
    },
    Entry {
        name: "tpt-eng-cad",
        summary: "Solid modeling and CAD kernel behavior: solid primitives, boolean operations, B-Rep, feature modeling, metadata",
        keywords: &["cad", "solid", "csg", "brep", "engineering"],
    },
    Entry {
        name: "tpt-eng-cli",
        summary: "Command-line front-end for the TPT engineering toolkit: unit conversion, material/section inspection, beam calculations, and file validation.",
        keywords: &["engineering", "cli", "units", "materials", "validation"],
    },
    Entry {
        name: "tpt-eng-controls",
        summary: "PID / state-space / transfer-function primitives.",
        keywords: &[
            "engineering",
            "control",
            "pid",
            "state-space",
            "transfer-function",
        ],
    },
    Entry {
        name: "tpt-eng-crystallography",
        summary: "Miller indices, slip systems, crystal symmetry operations.",
        keywords: &[
            "engineering",
            "crystallography",
            "miller-indices",
            "materials",
            "slip-systems",
        ],
    },
    Entry {
        name: "tpt-eng-electrical",
        summary: "Impedance/reactance, per-unit systems, three-phase power, conductor/insulator property lookups.",
        keywords: &[
            "engineering",
            "electrical",
            "impedance",
            "power-systems",
            "three-phase",
        ],
    },
    Entry {
        name: "tpt-eng-examples",
        summary: "Cross-crate integration scenarios composing the tpt-eng-* engineering primitives",
        keywords: &["engineering", "examples", "integration"],
    },
    Entry {
        name: "tpt-eng-gdt",
        summary: "Geometric dimensioning and tolerancing: datums, tolerance frames, tolerance zones, fits, allowances, stack-up inputs",
        keywords: &["gdt", "tolerance", "datum", "engineering", "manufacturing"],
    },
    Entry {
        name: "tpt-eng-geo-asset",
        summary: "Geographic asset registry: coordinates to logical device/node.",
        keywords: &[
            "geospatial",
            "gis",
            "assets",
            "coordinates",
            "infrastructure",
        ],
    },
    Entry {
        name: "tpt-eng-geo-topology",
        summary: "Directional infrastructure graphs (pipes/wires/ducts).",
        keywords: &[
            "graph",
            "topology",
            "infrastructure",
            "network",
            "traversal",
        ],
    },
    Entry {
        name: "tpt-eng-geometry",
        summary: "Core geometry primitives, frames, transforms, curves, surfaces, intersections, projections, and engineering geometry queries",
        keywords: &["geometry", "cad", "engineering", "3d", "primitives"],
    },
    Entry {
        name: "tpt-eng-geotech",
        summary: "Soil constitutive models, borehole stratigraphy.",
        keywords: &[
            "engineering",
            "geotechnical",
            "soil",
            "mohr-coulomb",
            "stratigraphy",
        ],
    },
    Entry {
        name: "tpt-eng-heat-transfer",
        summary: "Convection correlations, 1D/radial conduction, radiation view factors, thermal-resistance networks.",
        keywords: &[
            "engineering",
            "heat-transfer",
            "convection",
            "conduction",
            "thermal",
        ],
    },
    Entry {
        name: "tpt-eng-io",
        summary: "Engineering file I/O (STL and OBJ meshes) for the TPT ecosystem, built on `tpt-eng-mesh`.",
        keywords: &["engineering", "stl", "obj", "mesh", "io"],
    },
    Entry {
        name: "tpt-eng-materials",
        summary: "Material property modeling with source tracking, temperature dependence, anisotropic hooks, and original/openly-licensed data only.",
        keywords: &[
            "engineering",
            "materials",
            "material-properties",
            "thermodynamics",
            "data",
        ],
    },
    Entry {
        name: "tpt-eng-mesh",
        summary: "Mesh representation, normals, quality metrics, refinement/repair, and format-conversion hooks",
        keywords: &["mesh", "cad", "engineering", "geometry", "stl"],
    },
    Entry {
        name: "tpt-eng-network-matrix",
        summary: "Incidence/admittance matrix generation for downstream solvers.",
        keywords: &["matrix", "network", "incidence", "admittance", "laplacian"],
    },
    Entry {
        name: "tpt-eng-nurbs",
        summary: "Spline and NURBS modeling: B-splines, NURBS curves/surfaces, knot vectors, evaluation, derivatives, tessellation",
        keywords: &["nurbs", "bspline", "spline", "cad", "geometry"],
    },
    Entry {
        name: "tpt-eng-pcb",
        summary: "PCB layer stackup, trace routing primitives, via/footprint definitions.",
        keywords: &["engineering", "pcb", "electronics", "trace", "layout"],
    },
    Entry {
        name: "tpt-eng-plot",
        summary: "Plotting and diagram generation (PNG/SVG) for the TPT ecosystem, built on `plotters`.",
        keywords: &["engineering", "plot", "chart", "visualization", "svg"],
    },
    Entry {
        name: "tpt-eng-power-components",
        summary: "Transformer equivalent circuits, generator models, transmission line parameters.",
        keywords: &[
            "engineering",
            "power-systems",
            "transformer",
            "generator",
            "transmission-line",
        ],
    },
    Entry {
        name: "tpt-eng-props-air",
        summary: "ASHRAE psychrometrics / moist-air properties.",
        keywords: &[
            "engineering",
            "psychrometrics",
            "moist-air",
            "hvac",
            "ashrae",
        ],
    },
    Entry {
        name: "tpt-eng-props-fuels",
        summary: "Fuel properties: heating values, density, combustion.",
        keywords: &[
            "engineering",
            "fuels",
            "combustion",
            "heating-value",
            "energy",
        ],
    },
    Entry {
        name: "tpt-eng-props-mixture",
        summary: "General real-gas / VLE property lookups for arbitrary mixtures.",
        keywords: &["engineering", "thermodynamics", "real-gas", "vle", "fluids"],
    },
    Entry {
        name: "tpt-eng-props-water",
        summary: "IAPWS-IF97 water/steam property tables.",
        keywords: &[
            "engineering",
            "thermodynamics",
            "water",
            "steam",
            "properties",
        ],
    },
    Entry {
        name: "tpt-eng-props",
        summary: "Umbrella: re-exports water + air + fuels fluid properties.",
        keywords: &[
            "engineering",
            "thermodynamics",
            "properties",
            "fluids",
            "steam",
        ],
    },
    Entry {
        name: "tpt-eng-provenance",
        summary: "Provenance and validity-range tags for engineering results.",
        keywords: &["engineering", "provenance", "validity", "correlation"],
    },
    Entry {
        name: "tpt-eng-reliability",
        summary: "Reliability and life analysis: fatigue, life prediction, failure rates, FMEA, probabilistic design.",
        keywords: &[
            "engineering",
            "reliability",
            "maintenance",
            "weibull",
            "life-analysis",
        ],
    },
    Entry {
        name: "tpt-eng-renewables",
        summary: "Solar PV I-V curves, wind turbine power curves, battery degradation models.",
        keywords: &[
            "engineering",
            "renewable-energy",
            "solar",
            "wind",
            "battery",
        ],
    },
    Entry {
        name: "tpt-eng-report",
        summary: "Calculation-report data model and exporters (Markdown/HTML/JSON) for the TPT ecosystem.",
        keywords: &["engineering", "report", "markdown", "html", "export"],
    },
    Entry {
        name: "tpt-eng-safety",
        summary: "Safety margins and limit-state evaluation: utilization, margins, pass/fail, safety factors.",
        keywords: &[
            "safety",
            "engineering",
            "limit-state",
            "utilization",
            "margin",
        ],
    },
    Entry {
        name: "tpt-eng-schedule",
        summary: "CPM/PERT networks, resource leveling, Earned Value Management.",
        keywords: &[
            "engineering",
            "project-management",
            "cpm",
            "pert",
            "earned-value",
        ],
    },
    Entry {
        name: "tpt-eng-sections",
        summary: "Cross-section properties (area, centroid, second moment of area, section/plastic modulus, torsion) for common structural sections.",
        keywords: &[
            "engineering",
            "structural",
            "cross-section",
            "mechanics",
            "geometry",
        ],
    },
    Entry {
        name: "tpt-eng-sheet",
        summary: "Declarative calculation sheets: a TOML or JSON file names a calculation and its inputs, and produces a report.",
        keywords: &["engineering", "calculation", "toml", "report", "schema"],
    },
    Entry {
        name: "tpt-eng-standards",
        summary: "Engineering standards modeling as structured, parameterized data: load cases, combinations, factors, and limit-state logic (no copyrighted standard text).",
        keywords: &["engineering", "standards", "structural", "load", "design"],
    },
    Entry {
        name: "tpt-eng-structural",
        summary: "Structural engineering primitives: loads, beam/frame analysis, code checks.",
        keywords: &["structural", "engineering", "beam", "mechanics", "analysis"],
    },
    Entry {
        name: "tpt-eng-thermal-mgmt",
        summary: "Heat sink sizing, fan curves, thermal resistance networks.",
        keywords: &[
            "engineering",
            "thermal",
            "heat-sink",
            "cooling",
            "electronics",
        ],
    },
    Entry {
        name: "tpt-eng-timeseries-align",
        summary: "Irregular multi-rate stream alignment onto a unified time grid.",
        keywords: &["engineering", "timeseries", "signal-processing", "data"],
    },
    Entry {
        name: "tpt-eng-timeseries-core",
        summary: "Time-series core types.",
        keywords: &["engineering", "timeseries", "samples", "sensors", "data"],
    },
    Entry {
        name: "tpt-eng-timeseries-gap",
        summary: "Staleness/gap detection and interpolation for sensor dropouts.",
        keywords: &[
            "engineering",
            "timeseries",
            "interpolation",
            "sensors",
            "data",
        ],
    },
    Entry {
        name: "tpt-eng-timeseries",
        summary: "Umbrella: re-exports timeseries core + align + gap.",
        keywords: &["engineering", "timeseries", "sensors", "alignment", "data"],
    },
    Entry {
        name: "tpt-eng-tolerance",
        summary: "Tolerance analysis: worst-case, RSS, and Monte Carlo stack-up, sensitivity, and ranking.",
        keywords: &["engineering", "tolerance", "stack-up", "gdt", "analysis"],
    },
    Entry {
        name: "tpt-eng-unit-ops",
        summary: "Distillation stages, heat exchanger LMTD/epsilon-NTU, pump/compressor curves.",
        keywords: &[
            "engineering",
            "process",
            "distillation",
            "heat-exchanger",
            "pump",
        ],
    },
    Entry {
        name: "tpt-eng-vehicle-dynamics",
        summary: "Tire models (Pacejka), aerodynamic drag, suspension kinematics.",
        keywords: &[
            "engineering",
            "vehicle-dynamics",
            "tire",
            "aerodynamics",
            "suspension",
        ],
    },
];

/// Catalog entries matching every word of `query` (case-insensitive) in the
/// name, summary or keywords. With no words, every entry is returned. Results
/// keep catalog order.
pub fn search(query: &str) -> Vec<&'static Entry> {
    let words: Vec<String> = query
        .split_whitespace()
        .map(|w| w.to_ascii_lowercase())
        .collect();
    CATALOG
        .iter()
        .filter(|e| {
            words.iter().all(|w| {
                e.name.contains(w.as_str())
                    || e.summary.to_ascii_lowercase().contains(w.as_str())
                    || e.keywords.iter().any(|k| k.contains(w.as_str()))
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn catalog_lists_every_publishable_crate() {
        let crates = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
        let mut missing = Vec::new();
        for entry in std::fs::read_dir(&crates).expect("crates dir") {
            let dir = entry.expect("dir entry").path();
            let Ok(manifest) = std::fs::read_to_string(dir.join("Cargo.toml")) else {
                continue;
            };
            let name = manifest
                .lines()
                .find_map(|l| l.strip_prefix("name = \""))
                .map(|n| n.trim_end_matches('"').to_string());
            let Some(name) = name else { continue };
            if !name.starts_with("tpt-eng-") || manifest.contains("publish = false") {
                continue;
            }
            if !CATALOG.iter().any(|e| e.name == name) {
                missing.push(name);
            }
        }
        assert!(missing.is_empty(), "not in catalog: {missing:?}");
    }

    #[test]
    fn catalog_names_are_unique() {
        let mut names: Vec<&str> = CATALOG.iter().map(|e| e.name).collect();
        names.sort_unstable();
        let before = names.len();
        names.dedup();
        assert_eq!(before, names.len());
    }

    #[test]
    fn search_matches_all_words() {
        let hits = search("pid control");
        assert!(hits.iter().any(|e| e.name == "tpt-eng-controls"));
        assert!(search("zzz-no-such-thing").is_empty());
        assert_eq!(search("").len(), CATALOG.len());
    }
}
