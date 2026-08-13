# tpt-eng-controls

Control-systems primitives: a discrete-time [PID controller](Pid) (with
derivative-on-measurement and clamped-integral anti-windup), a discrete
[transfer function](TransferFunction), and a [state-space](StateSpace) model
with a forward-Euler step.

## Example

```rust
use tpt_eng_controls::{Pid, PidGains};

let mut pid = Pid::new(PidGains::new(2.0, 1.0, 0.0));
pid.set_setpoint(10.0);
let mut y = 0.0; // first-order plant, τ = 1 s
for _ in 0..600 {
    let u = pid.update(y, 0.01);
    y += 0.01 * u;
}
assert!((y - 10.0).abs() < 0.5);
```

Dual-licensed under MIT OR Apache-2.0. Copyright (c) 2026 TPT Solutions.
