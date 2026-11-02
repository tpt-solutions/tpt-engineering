//! Library calculations exposed as MCP tools, alongside calculation sheets.
//!
//! Each tool takes plain JSON arguments, checks them, and returns a JSON object
//! as text. Problems with the arguments come back as tool errors, so a client
//! can read the message and correct the call.

use serde_json::{Value, json};
use tpt_eng_props_air as air;
use tpt_eng_props_fuels::{BlendedFuel, Fuel};
use tpt_eng_tolerance::{DimTol, rank_contributors, rss, worst_case};
use tpt_math_units::uom::si::f64::{Pressure, ThermodynamicTemperature};
use tpt_math_units::uom::si::mass_density::kilogram_per_cubic_meter;
use tpt_math_units::uom::si::pressure::pascal;
use tpt_math_units::uom::si::thermodynamic_temperature::kelvin;

/// Largest number of dimensions a stack-up may contain.
const MAX_DIMENSIONS: usize = 1000;

/// Descriptors for `tools/list`.
pub fn tool_definitions() -> Vec<Value> {
    vec![
        json!({
            "name": "tolerance_stackup",
            "description": "Worst-case and root-sum-square (RSS) limits of a linear \
                dimension chain, in millimetres. Each dimension is nominal ± tolerance. \
                The result also ranks each dimension by its share of the RSS variance.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "dimensions": {
                        "type": "array",
                        "minItems": 1,
                        "maxItems": MAX_DIMENSIONS,
                        "items": {
                            "type": "object",
                            "properties": {
                                "name": { "type": "string" },
                                "nominal_mm": { "type": "number" },
                                "tolerance_mm": { "type": "number", "minimum": 0 }
                            },
                            "required": ["name", "nominal_mm", "tolerance_mm"],
                            "additionalProperties": false
                        }
                    }
                },
                "required": ["dimensions"],
                "additionalProperties": false
            },
            "annotations": { "readOnlyHint": true },
        }),
        json!({
            "name": "fuel_properties",
            "description": "Heating values, density, stoichiometric air-fuel ratio and \
                CO2 factor for a fuel. For \"blend\", give h2_fraction: the hydrogen mole \
                fraction in natural gas, from 0 to 1.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "fuel": {
                        "type": "string",
                        "enum": ["methane", "hydrogen", "natural-gas", "diesel", "blend"]
                    },
                    "h2_fraction": { "type": "number", "minimum": 0, "maximum": 1 }
                },
                "required": ["fuel"],
                "additionalProperties": false
            },
            "annotations": { "readOnlyHint": true },
        }),
        json!({
            "name": "moist_air_state",
            "description": "Moist-air state from dry-bulb temperature, relative humidity \
                and total pressure: saturation and vapour pressure, humidity ratio, \
                enthalpy, and dew point. Temperatures outside the ASHRAE range \
                (-100 to 200 °C) are computed and flagged with in_range = false.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "temperature_c": { "type": "number" },
                    "relative_humidity": { "type": "number", "minimum": 0, "maximum": 1 },
                    "pressure_kpa": { "type": "number", "exclusiveMinimum": 0, "default": 101.325 }
                },
                "required": ["temperature_c", "relative_humidity"],
                "additionalProperties": false
            },
            "annotations": { "readOnlyHint": true },
        }),
    ]
}

/// Run a calculation tool by name. Returns `None` if the name is not one of
/// these tools, so the caller can try its own tools.
pub fn call(name: &str, arguments: &Value) -> Option<Result<Value, String>> {
    let result = match name {
        "tolerance_stackup" => tolerance_stackup(arguments),
        "fuel_properties" => fuel_properties(arguments),
        "moist_air_state" => moist_air_state(arguments),
        _ => return None,
    };
    Some(result)
}

fn number(args: &Value, key: &str) -> Result<f64, String> {
    let value = args
        .get(key)
        .and_then(Value::as_f64)
        .ok_or_else(|| format!("`{key}` must be a number"))?;
    if value.is_finite() {
        Ok(value)
    } else {
        Err(format!("`{key}` must be a finite number"))
    }
}

fn tolerance_stackup(args: &Value) -> Result<Value, String> {
    let entries = args
        .get("dimensions")
        .and_then(Value::as_array)
        .ok_or_else(|| "`dimensions` must be an array".to_owned())?;
    if entries.is_empty() {
        return Err("`dimensions` must contain at least one dimension".to_owned());
    }
    if entries.len() > MAX_DIMENSIONS {
        return Err(format!(
            "`dimensions` may contain at most {MAX_DIMENSIONS} items"
        ));
    }

    let mut dims = Vec::with_capacity(entries.len());
    for (i, entry) in entries.iter().enumerate() {
        let name = entry
            .get("name")
            .and_then(Value::as_str)
            .filter(|n| !n.is_empty())
            .ok_or_else(|| format!("dimension {i} needs a non-empty `name`"))?;
        let nominal = number(entry, "nominal_mm")?;
        let tolerance = number(entry, "tolerance_mm")?;
        if tolerance < 0.0 {
            return Err(format!("dimension `{name}` has a negative tolerance"));
        }
        dims.push(DimTol::new(name, nominal, tolerance));
    }

    let (wc_lo, wc_hi) = worst_case(&dims);
    let (rss_lo, rss_hi) = rss(&dims);
    let contributors: Vec<Value> = rank_contributors(&dims)
        .into_iter()
        .map(|(index, share)| {
            json!({
                "name": dims[index].name,
                "variance_share": share,
            })
        })
        .collect();

    Ok(json!({
        "nominal_mm": dims.iter().map(|d| d.nominal).sum::<f64>(),
        "worst_case_mm": { "min": wc_lo, "max": wc_hi },
        "rss_mm": { "min": rss_lo, "max": rss_hi },
        "contributors": contributors,
    }))
}

fn fuel_properties(args: &Value) -> Result<Value, String> {
    let name = args
        .get("fuel")
        .and_then(Value::as_str)
        .ok_or_else(|| "`fuel` must be a string".to_owned())?;
    let properties = match name {
        "methane" => single_fuel(Fuel::Methane),
        "hydrogen" => single_fuel(Fuel::Hydrogen),
        "natural-gas" => single_fuel(Fuel::NaturalGas),
        "diesel" => single_fuel(Fuel::Diesel),
        "blend" => {
            let h2 = number(args, "h2_fraction")?;
            if !(0.0..=1.0).contains(&h2) {
                return Err("`h2_fraction` must be between 0 and 1".to_owned());
            }
            let blend = BlendedFuel::new(h2);
            json!({
                "fuel": "blend",
                "h2_fraction": h2,
                "lhv_mj_kg": blend.lhv,
                "hhv_mj_kg": blend.hhv,
                "density_kg_m3": blend.density.get::<kilogram_per_cubic_meter>(),
                "stoichiometric_air_fuel_ratio": blend.stoichiometric_air_fuel_ratio(),
                "co2_kg_per_mj": blend.co2_kg_per_mj(),
            })
        }
        other => return Err(format!("unknown fuel `{other}`")),
    };
    Ok(properties)
}

fn single_fuel(fuel: Fuel) -> Value {
    let name = match fuel {
        Fuel::Methane => "methane",
        Fuel::Hydrogen => "hydrogen",
        Fuel::NaturalGas => "natural-gas",
        Fuel::Diesel => "diesel",
    };
    json!({
        "fuel": name,
        "lhv_mj_kg": fuel.lhv_mj_kg(),
        "hhv_mj_kg": fuel.hhv_mj_kg(),
        "density_kg_m3": fuel.density().get::<kilogram_per_cubic_meter>(),
        "stoichiometric_air_fuel_ratio": fuel.stoichiometric_air_fuel_ratio(),
        "co2_kg_per_mj": fuel.co2_kg_per_mj(),
    })
}

fn moist_air_state(args: &Value) -> Result<Value, String> {
    let temperature_c = number(args, "temperature_c")?;
    let relative_humidity = number(args, "relative_humidity")?;
    if !(0.0..=1.0).contains(&relative_humidity) {
        return Err("`relative_humidity` must be between 0 and 1".to_owned());
    }
    let pressure_kpa = match args.get("pressure_kpa") {
        None => 101.325,
        Some(_) => number(args, "pressure_kpa")?,
    };
    if pressure_kpa <= 0.0 {
        return Err("`pressure_kpa` must be greater than zero".to_owned());
    }

    let temperature = ThermodynamicTemperature::new::<kelvin>(temperature_c + 273.15);
    let total = Pressure::new::<pascal>(pressure_kpa * 1000.0);
    let saturation = air::saturation_pressure_water_tagged(temperature);
    let in_range = saturation.provenance.warnings.is_empty();
    let p_sat = saturation.value.get::<pascal>();
    let p_w = Pressure::new::<pascal>(relative_humidity * p_sat);

    let w = air::humidity_ratio(p_w, total).map_err(|e| format!("{e:?}"))?;
    let enthalpy = air::moist_air_enthalpy(temperature, w);
    let dew_point_c = air::dew_point(p_w)
        .map_err(|e| format!("dew point is not defined here: {e:?}"))?
        .get::<kelvin>()
        - 273.15;

    Ok(json!({
        "temperature_c": temperature_c,
        "relative_humidity": relative_humidity,
        "pressure_kpa": pressure_kpa,
        "saturation_pressure_pa": p_sat,
        "vapour_pressure_pa": p_w.get::<pascal>(),
        "humidity_ratio_kg_per_kg": w,
        "enthalpy_kj_per_kg_dry_air": enthalpy,
        "dew_point_c": dew_point_c,
        "in_range": in_range,
        "source": saturation.provenance.source.label(),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(a: f64, b: f64, tol: f64) {
        assert!((a - b).abs() < tol, "{a} != {b}");
    }

    #[test]
    fn tolerance_worst_case_and_rss_match_hand_calculation() {
        // 10 ± 0.1 and 5 ± 0.05: worst case 15 ± 0.15; RSS 15 ± sqrt(0.1² + 0.05²).
        let args = json!({ "dimensions": [
            { "name": "a", "nominal_mm": 10.0, "tolerance_mm": 0.1 },
            { "name": "b", "nominal_mm": 5.0, "tolerance_mm": 0.05 },
        ]});
        let r = call("tolerance_stackup", &args).unwrap().unwrap();
        assert_close(r["worst_case_mm"]["min"].as_f64().unwrap(), 14.85, 1e-9);
        assert_close(r["worst_case_mm"]["max"].as_f64().unwrap(), 15.15, 1e-9);
        let rss_half = (0.1_f64.powi(2) + 0.05_f64.powi(2)).sqrt();
        assert_close(r["rss_mm"]["max"].as_f64().unwrap(), 15.0 + rss_half, 1e-9);
        assert_eq!(r["contributors"][0]["name"], "a");
    }

    #[test]
    fn tolerance_rejects_bad_input() {
        let negative = json!({ "dimensions": [
            { "name": "a", "nominal_mm": 1.0, "tolerance_mm": -0.1 }
        ]});
        assert!(call("tolerance_stackup", &negative).unwrap().is_err());

        let empty = json!({ "dimensions": [] });
        assert!(call("tolerance_stackup", &empty).unwrap().is_err());

        let unnamed = json!({ "dimensions": [
            { "name": "", "nominal_mm": 1.0, "tolerance_mm": 0.1 }
        ]});
        assert!(call("tolerance_stackup", &unnamed).unwrap().is_err());
    }

    #[test]
    fn fuel_properties_for_pure_fuels_and_a_blend() {
        let methane = call("fuel_properties", &json!({ "fuel": "methane" }))
            .unwrap()
            .unwrap();
        assert_close(methane["lhv_mj_kg"].as_f64().unwrap(), 50.0, 1e-9);
        assert_close(
            methane["stoichiometric_air_fuel_ratio"].as_f64().unwrap(),
            17.24,
            0.01,
        );

        let blend = call(
            "fuel_properties",
            &json!({ "fuel": "blend", "h2_fraction": 0.2 }),
        )
        .unwrap()
        .unwrap();
        assert!(blend["lhv_mj_kg"].as_f64().unwrap() > 50.0);
    }

    #[test]
    fn fuel_properties_rejects_unknown_fuel_and_bad_fraction() {
        assert!(
            call("fuel_properties", &json!({ "fuel": "coal" }))
                .unwrap()
                .is_err()
        );
        assert!(
            call(
                "fuel_properties",
                &json!({ "fuel": "blend", "h2_fraction": 1.5 })
            )
            .unwrap()
            .is_err()
        );
    }

    #[test]
    fn moist_air_at_20c_half_saturation() {
        let r = call(
            "moist_air_state",
            &json!({ "temperature_c": 20.0, "relative_humidity": 0.5 }),
        )
        .unwrap()
        .unwrap();
        // psat at 20 °C is about 2339 Pa, so the vapour pressure is half of it.
        assert_close(r["saturation_pressure_pa"].as_f64().unwrap(), 2339.0, 3.0);
        assert_close(r["vapour_pressure_pa"].as_f64().unwrap(), 1169.5, 2.0);
        assert_eq!(r["in_range"], true);
        // The dew point of 50 % RH at 20 °C is about 9.3 °C.
        assert_close(r["dew_point_c"].as_f64().unwrap(), 9.3, 0.2);
    }

    #[test]
    fn moist_air_flags_temperature_outside_the_correlation() {
        // At 1 atm, 250 °C is far above the boiling point, so a valid state needs a
        // higher total pressure. 1 % RH keeps the dew point inside the search range.
        let r = call(
            "moist_air_state",
            &json!({
                "temperature_c": 250.0,
                "relative_humidity": 0.01,
                "pressure_kpa": 5000.0
            }),
        )
        .unwrap()
        .unwrap();
        assert_eq!(r["in_range"], false);
    }

    #[test]
    fn moist_air_refuses_states_that_are_not_physical() {
        // Vapour cannot exceed the total pressure: 250 °C at 1 atm, 50 % RH.
        let r = call(
            "moist_air_state",
            &json!({ "temperature_c": 250.0, "relative_humidity": 0.5 }),
        )
        .unwrap();
        assert!(r.is_err());
    }

    #[test]
    fn moist_air_rejects_bad_humidity_and_pressure() {
        let high = json!({ "temperature_c": 20.0, "relative_humidity": 1.2 });
        assert!(call("moist_air_state", &high).unwrap().is_err());
        let zero_pressure = json!({
            "temperature_c": 20.0, "relative_humidity": 0.5, "pressure_kpa": 0.0
        });
        assert!(call("moist_air_state", &zero_pressure).unwrap().is_err());
    }

    #[test]
    fn unknown_name_is_not_handled_here() {
        assert!(call("run_sheet", &json!({})).is_none());
    }
}
