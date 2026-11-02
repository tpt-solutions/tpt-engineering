# tpt-eng-wasm

WebAssembly bindings and three browser demos for tpt-engineering:

| Demo | JavaScript API | Crate behind it |
| --- | --- | --- |
| Steam tables | `steam_state(T_K, p_Pa)` | `tpt-eng-props-water` with provenance tags |
| Section properties | `rectangle_section`, `circle_section`, `i_section` | `tpt-eng-sections` |
| PID step-response tuner | `pid_step_response(kp, ki, kd, setpoint, tau_s, dt_s, duration_s, output_limit?)` | `tpt-eng-controls` |

Inputs are SI. Invalid inputs throw a JavaScript `Error`. Out-of-range steam
states are returned with `in_range === false`, not rejected.

## Browser demo

```bash
cd crates/tpt-eng-wasm
wasm-pack build --target web --out-dir www/pkg
cd www
python -m http.server 8000   # ES modules need HTTP, not file://
```

Open <http://localhost:8000>. Nothing is sent to a server: every calculation
runs in the browser.

## Node.js smoke test

```bash
cd crates/tpt-eng-wasm
wasm-pack build --target nodejs --out-dir pkg-node
node tests/smoke.js
```

`www/pkg/` and `pkg-node/` are build output and are git-ignored.
