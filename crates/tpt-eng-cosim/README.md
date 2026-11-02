# tpt-eng-cosim

Fixed-step co-simulation of named models connected port to port. The interface
follows the FMI co-simulation pattern: set inputs, read outputs, step by a fixed
interval.

## Model interface

Implement `Model` to add a component. A model declares its input and output
ports by name, sets inputs, reads outputs, and advances its state by `dt`.

Built-in models:

| Model | Inputs | Outputs | Behaviour |
| --- | --- | --- | --- |
| `Constant` | none | `output` | fixed value, for setpoints and disturbances |
| `FirstOrderLag` | `input` | `output` | `τ·y' = K·u − y`, integrated exactly for a constant input over each step |
| `PidController` | `setpoint`, `measurement` | `control` | discrete PID from `tpt-eng-controls` |

## Coupling rule

Connections are explicit. At each step every input receives the value its
source output had at the start of the step, and then all models advance by
`dt`. A signal therefore reaches the next model one step late. Keep `dt` small
against the models' time constants.

## Example

```rust
use tpt_eng_cosim::{Constant, FirstOrderLag, PidController, Simulation};
use tpt_eng_controls::PidGains;

let mut sim = Simulation::new();
sim.add_model("setpoint", Constant::new(1.0))?;
sim.add_model("pid", PidController::new(PidGains::new(2.0, 1.0, 0.0)))?;
sim.add_model("plant", FirstOrderLag::new(1.0, 1.0)?)?;
sim.connect(("setpoint", "output"), ("pid", "setpoint"))?;
sim.connect(("plant", "output"), ("pid", "measurement"))?;
sim.connect(("pid", "control"), ("plant", "input"))?;

let trace = sim.run(0.01, 2000)?;
let y = trace.signal("plant.output").unwrap();
```

`Trace` holds the time points and every output port, named `model.port`.

## Status

Not published. The `Model` trait will change as more models are added, so it
stays out of crates.io until the interface settles.
