//! # tpt-eng-cosim
//!
//! Fixed-step co-simulation. Independent models, each with named input and
//! output ports, are connected port to port and advanced together.
//!
//! The interface follows the FMI co-simulation pattern: set inputs, read
//! outputs, and step by a fixed interval. Each [`Model`] owns its state.
//!
//! ## Coupling rule
//!
//! Connections are explicit. At every step, each input receives the value its
//! source output had at the start of the step, and then every model advances by
//! `dt`. A signal therefore reaches the next model one step late. The result is
//! stable for small enough `dt` relative to the models' time constants. A model
//! that feeds itself back through a connection is handled the same way.
//!
//! ## Example
//!
//! A PI loop around a first-order plant, with a constant setpoint:
//!
//! ```
//! use tpt_eng_cosim::{Constant, FirstOrderLag, PidController, Simulation};
//! use tpt_eng_controls::PidGains;
//!
//! let mut sim = Simulation::new();
//! sim.add_model("setpoint", Constant::new(1.0)).unwrap();
//! sim.add_model("pid", PidController::new(PidGains::new(2.0, 1.0, 0.0))).unwrap();
//! sim.add_model("plant", FirstOrderLag::new(1.0, 1.0).unwrap()).unwrap();
//! sim.connect(("setpoint", "output"), ("pid", "setpoint")).unwrap();
//! sim.connect(("plant", "output"), ("pid", "measurement")).unwrap();
//! sim.connect(("pid", "control"), ("plant", "input")).unwrap();
//!
//! let trace = sim.run(0.01, 2000).unwrap();
//! let y = trace.signal("plant.output").unwrap();
//! assert!((y[y.len() - 1] - 1.0).abs() < 0.01);
//! ```

use std::fmt;

use tpt_eng_controls::{Pid, PidGains};

/// A model with named input and output ports, advanced in fixed steps.
///
/// Port names are static, so every port is known before the model is added.
pub trait Model {
    /// Names of the input ports.
    fn inputs(&self) -> &'static [&'static str];

    /// Names of the output ports.
    fn outputs(&self) -> &'static [&'static str];

    /// Set an input port. Values take effect at the next [`Model::step`].
    ///
    /// # Errors
    ///
    /// Returns [`CosimError::UnknownPort`] if the model has no such input.
    fn set_input(&mut self, port: &str, value: f64) -> Result<(), CosimError>;

    /// Read an output port at the model's current time.
    ///
    /// # Errors
    ///
    /// Returns [`CosimError::UnknownPort`] if the model has no such output.
    fn output(&self, port: &str) -> Result<f64, CosimError>;

    /// Advance the model by `dt` seconds, using the inputs set since the last
    /// step.
    ///
    /// # Errors
    ///
    /// Returns [`CosimError::InvalidStep`] if `dt` is not a positive finite number.
    fn step(&mut self, dt: f64) -> Result<(), CosimError>;
}

/// Errors from building or running a simulation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CosimError {
    /// A model name is already in use.
    DuplicateModel(String),
    /// No model has this name.
    UnknownModel(String),
    /// The model has no port with this name.
    UnknownPort { model: String, port: String },
    /// An input is already driven by another connection.
    InputAlreadyConnected { model: String, port: String },
    /// A step size or model parameter is not a valid positive number.
    InvalidStep(String),
    /// A model parameter is outside its valid range.
    InvalidParameter(String),
}

impl fmt::Display for CosimError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CosimError::DuplicateModel(m) => write!(f, "a model named `{m}` already exists"),
            CosimError::UnknownModel(m) => write!(f, "no model named `{m}`"),
            CosimError::UnknownPort { model, port } => {
                write!(f, "model `{model}` has no port `{port}`")
            }
            CosimError::InputAlreadyConnected { model, port } => {
                write!(f, "input `{model}.{port}` is already connected")
            }
            CosimError::InvalidStep(msg) => write!(f, "invalid step: {msg}"),
            CosimError::InvalidParameter(msg) => write!(f, "invalid parameter: {msg}"),
        }
    }
}

impl std::error::Error for CosimError {}

/// A directed link from one model's output to another model's input.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Connection {
    src: usize,
    src_port: String,
    dst: usize,
    dst_port: String,
}

/// Recorded output signals, sampled at every time point of a run.
#[derive(Debug, Clone, PartialEq)]
pub struct Trace {
    /// Time points in seconds, starting at 0.
    pub time: Vec<f64>,
    /// Signal names, in the form `model.port`, paired with their samples.
    signals: Vec<(String, Vec<f64>)>,
}

impl Trace {
    /// The samples of the named signal, such as `"plant.output"`.
    pub fn signal(&self, name: &str) -> Option<&[f64]> {
        self.signals
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, values)| values.as_slice())
    }

    /// Names of all recorded signals.
    pub fn signal_names(&self) -> impl Iterator<Item = &str> {
        self.signals.iter().map(|(n, _)| n.as_str())
    }
}

/// A set of named models and the connections between their ports.
#[derive(Default)]
pub struct Simulation {
    models: Vec<(String, Box<dyn Model>)>,
    connections: Vec<Connection>,
}

impl Simulation {
    /// An empty simulation.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a model under a unique name.
    ///
    /// # Errors
    ///
    /// Returns [`CosimError::DuplicateModel`] if the name is already used.
    pub fn add_model(
        &mut self,
        name: impl Into<String>,
        model: impl Model + 'static,
    ) -> Result<(), CosimError> {
        let name = name.into();
        if self.index_of(&name).is_ok() {
            return Err(CosimError::DuplicateModel(name));
        }
        self.models.push((name, Box::new(model)));
        Ok(())
    }

    /// Connect an output port to an input port, each given as
    /// `(model name, port name)`.
    ///
    /// # Errors
    ///
    /// Returns an error if a model or port does not exist, or if the input is
    /// already driven by another connection.
    pub fn connect(&mut self, from: (&str, &str), to: (&str, &str)) -> Result<(), CosimError> {
        let (src, src_port) = self.resolve_output(from.0, from.1)?;
        let (dst, dst_port) = self.resolve_input(to.0, to.1)?;
        if self
            .connections
            .iter()
            .any(|c| c.dst == dst && c.dst_port == dst_port)
        {
            return Err(CosimError::InputAlreadyConnected {
                model: to.0.to_owned(),
                port: to.1.to_owned(),
            });
        }
        self.connections.push(Connection {
            src,
            src_port,
            dst,
            dst_port,
        });
        Ok(())
    }

    /// Run for `steps` fixed steps of `dt` seconds, recording every output
    /// at time 0 and after each step.
    ///
    /// # Errors
    ///
    /// Returns [`CosimError::InvalidStep`] if `dt` is not a positive finite
    /// number, and propagates any error from the models.
    pub fn run(&mut self, dt: f64, steps: usize) -> Result<Trace, CosimError> {
        if !(dt.is_finite() && dt > 0.0) {
            return Err(CosimError::InvalidStep(format!(
                "dt must be a positive finite number, got {dt}"
            )));
        }
        let names = self.signal_names();
        let mut signals: Vec<(String, Vec<f64>)> =
            names.iter().map(|n| (n.clone(), Vec::new())).collect();
        let mut time = Vec::with_capacity(steps + 1);
        let mut t = 0.0;
        self.record(&mut time, &mut signals, t)?;

        for _ in 0..steps {
            self.propagate()?;
            for (_, model) in &mut self.models {
                model.step(dt)?;
            }
            t += dt;
            self.record(&mut time, &mut signals, t)?;
        }
        Ok(Trace { time, signals })
    }

    fn index_of(&self, name: &str) -> Result<usize, CosimError> {
        self.models
            .iter()
            .position(|(n, _)| n == name)
            .ok_or_else(|| CosimError::UnknownModel(name.to_owned()))
    }

    fn resolve_output(&self, model: &str, port: &str) -> Result<(usize, String), CosimError> {
        let index = self.index_of(model)?;
        let model_ref = &self.models[index].1;
        if model_ref.outputs().contains(&port) {
            Ok((index, port.to_owned()))
        } else {
            Err(CosimError::UnknownPort {
                model: model.to_owned(),
                port: port.to_owned(),
            })
        }
    }

    fn resolve_input(&self, model: &str, port: &str) -> Result<(usize, String), CosimError> {
        let index = self.index_of(model)?;
        let model_ref = &self.models[index].1;
        if model_ref.inputs().contains(&port) {
            Ok((index, port.to_owned()))
        } else {
            Err(CosimError::UnknownPort {
                model: model.to_owned(),
                port: port.to_owned(),
            })
        }
    }

    /// Every output port of every model, as `model.port` names, in a fixed order.
    fn signal_names(&self) -> Vec<String> {
        self.models
            .iter()
            .flat_map(|(name, model)| {
                model
                    .outputs()
                    .iter()
                    .map(move |port| format!("{name}.{port}"))
            })
            .collect()
    }

    /// Copy each connected output into its input. The values are read first, so
    /// the order of the connections does not matter.
    fn propagate(&mut self) -> Result<(), CosimError> {
        let values: Vec<f64> = self
            .connections
            .iter()
            .map(|c| self.models[c.src].1.output(&c.src_port))
            .collect::<Result<_, _>>()?;
        for (c, value) in self.connections.iter().zip(values) {
            self.models[c.dst].1.set_input(&c.dst_port, value)?;
        }
        Ok(())
    }

    fn record(
        &self,
        time: &mut Vec<f64>,
        signals: &mut [(String, Vec<f64>)],
        t: f64,
    ) -> Result<(), CosimError> {
        time.push(t);
        let mut slot = 0;
        for (name, model) in &self.models {
            for port in model.outputs() {
                let value = model.output(port)?;
                signals[slot].1.push(value);
                debug_assert_eq!(signals[slot].0, format!("{name}.{port}"));
                slot += 1;
            }
        }
        Ok(())
    }
}

/// A model that outputs a fixed value. Useful as a setpoint or a disturbance.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Constant {
    value: f64,
}

impl Constant {
    /// A constant source with the given value.
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

impl Model for Constant {
    fn inputs(&self) -> &'static [&'static str] {
        &[]
    }

    fn outputs(&self) -> &'static [&'static str] {
        &["output"]
    }

    fn set_input(&mut self, port: &str, _value: f64) -> Result<(), CosimError> {
        Err(unknown_port("constant", port))
    }

    fn output(&self, port: &str) -> Result<f64, CosimError> {
        match port {
            "output" => Ok(self.value),
            other => Err(unknown_port("constant", other)),
        }
    }

    fn step(&mut self, dt: f64) -> Result<(), CosimError> {
        check_dt(dt)
    }
}

/// A first-order lag, `τ·y' = K·u − y`, integrated exactly for a constant input
/// over each step.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FirstOrderLag {
    tau: f64,
    gain: f64,
    input: f64,
    output: f64,
}

impl FirstOrderLag {
    /// A lag with time constant `tau` (seconds, positive) and static gain `gain`.
    ///
    /// # Errors
    ///
    /// Returns [`CosimError::InvalidParameter`] if `tau` is not positive and
    /// finite, or `gain` is not finite.
    pub fn new(tau: f64, gain: f64) -> Result<Self, CosimError> {
        if !(tau.is_finite() && tau > 0.0) {
            return Err(CosimError::InvalidParameter(format!(
                "tau must be positive and finite, got {tau}"
            )));
        }
        if !gain.is_finite() {
            return Err(CosimError::InvalidParameter(format!(
                "gain must be finite, got {gain}"
            )));
        }
        Ok(Self {
            tau,
            gain,
            input: 0.0,
            output: 0.0,
        })
    }
}

impl Model for FirstOrderLag {
    fn inputs(&self) -> &'static [&'static str] {
        &["input"]
    }

    fn outputs(&self) -> &'static [&'static str] {
        &["output"]
    }

    fn set_input(&mut self, port: &str, value: f64) -> Result<(), CosimError> {
        match port {
            "input" => {
                self.input = value;
                Ok(())
            }
            other => Err(unknown_port("first-order-lag", other)),
        }
    }

    fn output(&self, port: &str) -> Result<f64, CosimError> {
        match port {
            "output" => Ok(self.output),
            other => Err(unknown_port("first-order-lag", other)),
        }
    }

    fn step(&mut self, dt: f64) -> Result<(), CosimError> {
        check_dt(dt)?;
        let target = self.gain * self.input;
        self.output += (target - self.output) * (1.0 - (-dt / self.tau).exp());
        Ok(())
    }
}

/// A discrete PID controller as a model. Inputs are `setpoint` and
/// `measurement`, and the output is `control`.
pub struct PidController {
    pid: Pid,
    measurement: f64,
    control: f64,
}

impl PidController {
    /// A PID model with the given gains and no output limit.
    pub fn new(gains: PidGains) -> Self {
        Self {
            pid: Pid::new(gains),
            measurement: 0.0,
            control: 0.0,
        }
    }
}

impl Model for PidController {
    fn inputs(&self) -> &'static [&'static str] {
        &["setpoint", "measurement"]
    }

    fn outputs(&self) -> &'static [&'static str] {
        &["control"]
    }

    fn set_input(&mut self, port: &str, value: f64) -> Result<(), CosimError> {
        match port {
            "setpoint" => {
                self.pid.set_setpoint(value);
                Ok(())
            }
            "measurement" => {
                self.measurement = value;
                Ok(())
            }
            other => Err(unknown_port("pid", other)),
        }
    }

    fn output(&self, port: &str) -> Result<f64, CosimError> {
        match port {
            "control" => Ok(self.control),
            other => Err(unknown_port("pid", other)),
        }
    }

    fn step(&mut self, dt: f64) -> Result<(), CosimError> {
        check_dt(dt)?;
        self.control = self.pid.update(self.measurement, dt);
        Ok(())
    }
}

fn unknown_port(model: &str, port: &str) -> CosimError {
    CosimError::UnknownPort {
        model: model.to_owned(),
        port: port.to_owned(),
    }
}

fn check_dt(dt: f64) -> Result<(), CosimError> {
    if dt.is_finite() && dt > 0.0 {
        Ok(())
    } else {
        Err(CosimError::InvalidStep(format!(
            "dt must be a positive finite number, got {dt}"
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pi_loop(setpoint: f64) -> Simulation {
        let mut sim = Simulation::new();
        sim.add_model("setpoint", Constant::new(setpoint)).unwrap();
        sim.add_model("pid", PidController::new(PidGains::new(2.0, 1.0, 0.0)))
            .unwrap();
        sim.add_model("plant", FirstOrderLag::new(1.0, 1.0).unwrap())
            .unwrap();
        sim.connect(("setpoint", "output"), ("pid", "setpoint"))
            .unwrap();
        sim.connect(("plant", "output"), ("pid", "measurement"))
            .unwrap();
        sim.connect(("pid", "control"), ("plant", "input")).unwrap();
        sim
    }

    #[test]
    fn closed_loop_reaches_setpoint() {
        let trace = pi_loop(2.0).run(0.01, 3000).unwrap();
        let y = trace.signal("plant.output").unwrap();
        assert!((y[y.len() - 1] - 2.0).abs() < 0.01);
    }

    #[test]
    fn trace_has_one_sample_per_time_point() {
        let trace = pi_loop(1.0).run(0.05, 100).unwrap();
        assert_eq!(trace.time.len(), 101);
        assert_eq!(trace.signal("pid.control").unwrap().len(), 101);
        assert!((trace.time[100] - 5.0).abs() < 1e-12);
    }

    #[test]
    fn signal_names_are_model_dot_port() {
        let trace = pi_loop(1.0).run(0.1, 1).unwrap();
        let mut names: Vec<&str> = trace.signal_names().collect();
        names.sort_unstable();
        assert_eq!(names, ["pid.control", "plant.output", "setpoint.output"]);
    }

    #[test]
    fn lag_matches_exact_step_response() {
        // Constant input 1 on a unit lag: y(t) = 1 − e^(−t). The exact step makes
        // the only error the one-step connection delay, which is dt/2 at most.
        let mut sim = Simulation::new();
        sim.add_model("u", Constant::new(1.0)).unwrap();
        sim.add_model("lag", FirstOrderLag::new(1.0, 1.0).unwrap())
            .unwrap();
        sim.connect(("u", "output"), ("lag", "input")).unwrap();
        let trace = sim.run(0.001, 1000).unwrap();
        let y = trace.signal("lag.output").unwrap();
        let t = trace.time[1000];
        assert!((y[1000] - (1.0 - (-t).exp())).abs() < 1e-9);
    }

    #[test]
    fn connection_errors_are_reported() {
        let mut sim = Simulation::new();
        sim.add_model("a", Constant::new(1.0)).unwrap();
        sim.add_model("b", FirstOrderLag::new(1.0, 1.0).unwrap())
            .unwrap();

        assert!(matches!(
            sim.add_model("a", Constant::new(2.0)),
            Err(CosimError::DuplicateModel(_))
        ));
        assert!(matches!(
            sim.connect(("missing", "output"), ("b", "input")),
            Err(CosimError::UnknownModel(_))
        ));
        assert!(matches!(
            sim.connect(("a", "nope"), ("b", "input")),
            Err(CosimError::UnknownPort { .. })
        ));
        sim.connect(("a", "output"), ("b", "input")).unwrap();
        assert!(matches!(
            sim.connect(("a", "output"), ("b", "input")),
            Err(CosimError::InputAlreadyConnected { .. })
        ));
    }

    #[test]
    fn invalid_step_and_parameters_are_rejected() {
        let mut sim = pi_loop(1.0);
        assert!(matches!(sim.run(0.0, 10), Err(CosimError::InvalidStep(_))));
        assert!(matches!(
            sim.run(f64::NAN, 10),
            Err(CosimError::InvalidStep(_))
        ));
        assert!(matches!(
            FirstOrderLag::new(0.0, 1.0),
            Err(CosimError::InvalidParameter(_))
        ));
        assert!(matches!(
            FirstOrderLag::new(1.0, f64::INFINITY),
            Err(CosimError::InvalidParameter(_))
        ));
    }
}
