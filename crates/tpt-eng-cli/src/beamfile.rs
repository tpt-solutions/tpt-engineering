//! File-driven beam analysis: `tpt-eng calc beam-file beam.json`.
//!
//! Input (all lengths in m, forces in kN, moments in kN·m):
//!
//! ```json
//! {
//!   "span_m": 10.0,
//!   "loads": [
//!     { "type": "point", "at_m": 5.0, "kn": 10.0 },
//!     { "type": "uniform", "start_m": 0.0, "end_m": 4.0, "kn": 8.0 },
//!     { "type": "moment", "at_m": 2.0, "knm": 3.0 }
//!   ],
//!   "section_modulus_m3": 1.0e-4,
//!   "allowable_stress_mpa": 250.0
//! }
//! ```
//!
//! The section fields are optional. With both, the output includes a
//! utilisation ratio for the maximum moment.

use std::path::Path;

use anyhow::{Context, Result, anyhow, bail};
use serde_json::{Value, json};
use tpt_eng_structural::{Beam, Load, SectionCheck};
use tpt_math_units::uom::si::f64::{Force, Length, Pressure, Torque, Volume};
use tpt_math_units::uom::si::{
    force::kilonewton, length::meter, pressure::megapascal, torque::kilonewton_meter,
    volume::cubic_meter,
};

/// Read a number field, with a message naming the field when it is missing.
fn number(v: &Value, key: &str) -> Result<f64> {
    v.get(key)
        .and_then(Value::as_f64)
        .ok_or_else(|| anyhow!("missing or non-numeric field `{key}`"))
}

/// Build a [`Beam`] from the parsed JSON document.
fn beam_from_json(doc: &Value) -> Result<Beam> {
    let span = number(doc, "span_m")?;
    let mut beam = Beam::new(Length::new::<meter>(span));
    let loads = doc
        .get("loads")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("`loads` must be an array"))?;
    for (i, item) in loads.iter().enumerate() {
        let kind = item
            .get("type")
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow!("load {i}: missing `type`"))?;
        let load = match kind {
            "point" => Load::point(
                Length::new::<meter>(number(item, "at_m")?),
                Force::new::<kilonewton>(number(item, "kn")?),
            ),
            "uniform" => Load::uniform(
                Length::new::<meter>(number(item, "start_m")?),
                Length::new::<meter>(number(item, "end_m")?),
                Force::new::<kilonewton>(number(item, "kn")?),
            ),
            "moment" => Load::moment(
                Length::new::<meter>(number(item, "at_m")?),
                Torque::new::<kilonewton_meter>(number(item, "knm")?),
            ),
            other => bail!("load {i}: unknown type `{other}` (point, uniform, moment)"),
        };
        beam.try_add(load)
            .map_err(|e| anyhow!("load {i}: {e}"))
            .with_context(|| format!("while adding load {i}"))?;
    }
    Ok(beam)
}

/// Analyse the beam described in `path` and return the JSON result document.
pub fn analyse(path: &Path) -> Result<Value> {
    let text =
        std::fs::read_to_string(path).with_context(|| format!("cannot read {}", path.display()))?;
    let doc: Value = serde_json::from_str(&text).context("beam file is not valid JSON")?;
    let beam = beam_from_json(&doc)?;

    let moment = beam.max_bending_moment();
    let mut out = json!({
        "reaction_a_kn": beam.reaction_a().get::<kilonewton>(),
        "reaction_b_kn": beam.reaction_b().get::<kilonewton>(),
        "max_moment_knm": moment.get::<kilonewton_meter>(),
    });

    if let (Some(z), Some(sigma)) = (
        doc.get("section_modulus_m3").and_then(Value::as_f64),
        doc.get("allowable_stress_mpa").and_then(Value::as_f64),
    ) {
        let check = SectionCheck::new(
            Volume::new::<cubic_meter>(z),
            Pressure::new::<megapascal>(sigma),
        );
        out["utilization"] = json!(check.utilization(moment));
    }
    Ok(out)
}

/// Print the analysis as pretty JSON.
pub fn run(path: &Path) -> Result<()> {
    let out = analyse(path)?;
    println!("{}", serde_json::to_string_pretty(&out)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_temp(name: &str, body: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!("tpt-eng-{name}.json"));
        std::fs::write(&path, body).unwrap();
        path
    }

    #[test]
    fn central_point_load_matches_closed_form() {
        let path = write_temp(
            "central",
            r#"{"span_m": 10.0, "loads": [{"type": "point", "at_m": 5.0, "kn": 10.0}]}"#,
        );
        let out = analyse(&path).unwrap();
        assert!((out["reaction_a_kn"].as_f64().unwrap() - 5.0).abs() < 1e-9);
        assert!((out["max_moment_knm"].as_f64().unwrap() - 25.0).abs() < 1e-6);
        assert!(out.get("utilization").is_none());
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn section_adds_utilization() {
        // M = 25 kN·m, Z = 1e-4 m³, σ = 250 MPa → U = 1.0.
        let path = write_temp(
            "section",
            r#"{"span_m": 10.0, "loads": [{"type": "point", "at_m": 5.0, "kn": 10.0}],
                "section_modulus_m3": 1e-4, "allowable_stress_mpa": 250.0}"#,
        );
        let out = analyse(&path).unwrap();
        assert!((out["utilization"].as_f64().unwrap() - 1.0).abs() < 1e-9);
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn bad_load_is_reported_with_its_index() {
        let path = write_temp(
            "bad",
            r#"{"span_m": 10.0, "loads": [{"type": "point", "at_m": 12.0, "kn": 1.0}]}"#,
        );
        let err = format!("{:#}", analyse(&path).unwrap_err());
        assert!(err.contains("load 0"), "{err}");
        let _ = std::fs::remove_file(path);
    }
}
