// Smoke test for the Node.js build of the WASM package. CI runs:
//   wasm-pack build --target nodejs --out-dir pkg-node
//   node tests/smoke.js
const assert = require("node:assert/strict");
const w = require("../pkg-node/tpt_eng_wasm.js");

// Steam tables: IF97 reference point at 300 K, 3 MPa.
const s = w.steam_state(300, 3e6);
assert.equal(s.region, "liquid");
assert.ok(Math.abs(s.enthalpy_kj_per_kg - 115.331) < 1e-2);
assert.equal(s.in_range, true);
assert.equal(w.steam_state(1100, 1e5).in_range, false);
assert.throws(() => w.steam_state(200, 1e5), /273\.15/);

// Section properties.
const r = w.rectangle_section(0.1, 0.2);
assert.ok(Math.abs(r.area - 0.02) < 1e-12);
assert.ok(Math.abs(r.second_moment_x - (0.1 * 0.2 ** 3) / 12) < 1e-12);
const c = w.circle_section(0.1);
assert.ok(Math.abs(c.area - Math.PI * 0.05 ** 2) < 1e-9);
assert.throws(() => w.i_section(0.2, 0.1, 0.12, 0.005), /flange thicknesses/);
const i = w.i_section(0.2, 0.1, 0.01, 0.006);
assert.ok(i.second_moment_x > i.second_moment_y);

// PID tuner: PI loop reaches the setpoint; unstable step is rejected.
const y = w.pid_step_response(2, 1, 0, 1, 1, 0.01, 20, undefined);
assert.ok(y instanceof Float64Array);
assert.equal(y.length, 2001);
assert.ok(Math.abs(y[y.length - 1] - 1) < 0.02);
assert.throws(() => w.pid_step_response(1, 0, 0, 1, 0.5, 0.5, 10, undefined), /smaller than tau/);

console.log("tpt-eng-wasm Node smoke test passed");
