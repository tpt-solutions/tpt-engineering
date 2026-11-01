# Validation

This file records what the numerical crates are checked against. Each check
is either:

- **Independent**: the expected value comes from a source outside this
  workspace (a textbook closed form, a published table, or a hand calculation
  with the arithmetic shown in the test).
- **Regression**: the expected value comes from the implemented formula at
  a stated point. These catch unintended change. They do not show the formula
  is right; that rests on the cited source.

Test names are in the crate named in the first column. Run them with
`cargo test -p <crate>`.

## Independent checks

| Crate | Quantity | Expected | Source | Test |
|---|---|---|---|---|
| `tpt-eng-structural` | Simply supported, central point load: `M_max = PL/4` | 25 kN·m for 10 kN at 10 m | Standard beam formulas (e.g. Hibbeler, *Mechanics of Materials*) | `central_point_load_reactions_and_moment` |
| `tpt-eng-structural` | Full-span UDL: `M_max = wL²/8` | 16 kN·m for 2 kN/m over 8 m | Same | `udl_max_moment` |
| `tpt-eng-structural` | Partial UDL beyond its end: moment vanishes at the free support | 0 at x = L | Equilibrium | `partial_udl_moment_beyond_load_end` |
| `tpt-eng-electrical` | DC resistance `R = ρL/A` | 1.68 Ω for 1 km of 10 mm² copper | Hand calculation, `ρ_Cu` = 1.68·10⁻⁸ Ω·m | `dc_and_skin` |
| `tpt-eng-electrical` | Three-phase apparent power `S = √3·V·I` | 6235 W for 400 V, 10 A, pf 0.9 (real part) | Standard AC power relations | `three_phase` |
| `tpt-eng-props-water` | Saturation pressure at 100 °C | ≈ 101.3 kPa | IAPWS-IF97 (Wagner & Kruse, 1998) | `tpt-eng-props` doctest |
| `tpt-eng-props-air` | Over-ice saturation at −10 °C | ≈ 259.9 Pa | ASHRAE Fundamentals (Hyland–Wexler over-ice form) | `over_ice_branch_below_triple_point` |
| `tpt-eng-props-air` | Ice and liquid forms agree at the triple point | within 0.2 % | Continuity of the ASHRAE correlations | `over_ice_branch_below_triple_point` |
| `tpt-eng-controls` | First-order ZOH step response | `(1 − e^−0.1) z⁻¹ / (1 − e^−0.1 z⁻¹)` | Exact discretisation of `1/(s+1)` | `transfer_function_first_order_step` |
| `tpt-eng-geotech` | Mohr–Coulomb shear strength | `τ = c + σ tan φ` | Definition | `tpt-eng-geotech` doctest |
| `tpt-eng-thermal-mgmt` | Fin efficiency at `mL = 0.2` | ≈ 0.987 | `tanh(mL)/(mL)` | `tpt-eng-thermal-mgmt` doctest |
| `tpt-eng-reliability` | Miner's rule | `Σ n/N_f` (0.2 for the doctest blocks) | Miner (1945) | `miners_rule_sums` |

## Regression checks

These pin the implemented formula at a point. They do not confirm the formula
against a second source.

| Crate | Formula | Point | Test |
|---|---|---|---|
| `tpt-eng-heat-transfer` | Flat-plate laminar `Nu = 0.664·Re^½·Pr^⅓` | Re = 10⁵, Pr = 0.7 | `flat_plate_laminar` |
| `tpt-eng-heat-transfer` | Whitaker sphere `Nu = 2 + (0.4Re^½ + 0.06Re^⅔)·Pr^0.4·(μ/μ_s)^¼` | Re = 10³, Pr = 0.7 | `sphere_whitaker_viscosity_term` |
| `tpt-eng-unit-ops` | Eduljee–Gilliland `N` | N_min = 10, R_min = 1, R = 2 → 16.1 | `gilliland_eduljee_reference` |
| `tpt-eng-unit-ops` | Fenske minimum stages | 6.43 for x_d = 0.95, x_b = 0.05, α = 2.5 | `fenske_basics` |
| `tpt-eng-pcb` | Hammerstad–Jensen microstrip | 2.9 mm on 1.6 mm FR-4 → 50 ± 2 Ω | `microstrip_fifty_ohm_band` |
| `tpt-eng-renewables` | Single-diode solve | residual < 10⁻⁹ A across the I–V curve | `pv_solution_satisfies_diode_equation_near_voc` |
| `tpt-eng-tolerance` | Worst-case stack-up | 34.7–35.3 for 10 ± 0.1 and 25 ± 0.2 | `tpt-eng-tolerance` doctest |

## Not yet validated

- IAPWS-IF97 is checked at a few spot points, not against the full published
  table set. Region 3 is unsupported (see `tpt-eng-props-water`).
- A CSV of reference values with a citation per row (`tests/reference/`) is
  planned but not written. Until it is, the table above is the record.
- Skin-effect and microstrip closed forms are approximations. Their validity
  ranges are in the crate docs, not re-derived here.
