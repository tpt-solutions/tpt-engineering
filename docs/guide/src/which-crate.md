# Which crate do I need?

Choose by task. This page is generated from the README table, so the two
stay in step.

| If you need to… | Start with |
|-------|--------|
| Compute water/steam thermodynamic properties (IAPWS-IF97) | `tpt-eng-props-water` |
| Compute moist-air / psychrometric properties (ASHRAE) | `tpt-eng-props-air` |
| Compute fuel heating values, density, combustion properties | `tpt-eng-props-fuels` |
| Work with any/all fluid properties at once | `tpt-eng-props` (umbrella) |
| Condition/timestamp irregular sensor streams | `tpt-eng-timeseries-core` |
| Align multi-rate streams (e.g. 1 Hz CAN vs 10 s Modbus) onto one grid | `tpt-eng-timeseries-align` |
| Detect staleness/dropouts and interpolate gaps | `tpt-eng-timeseries-gap` |
| Use all time-series primitives together | `tpt-eng-timeseries` (umbrella) |
| Register geographic assets / devices | `tpt-eng-geo-asset` |
| Model pipes/wires/ducts as directed graphs | `tpt-eng-geo-topology` |
| Generate incidence/admittance matrices from topology | `tpt-eng-network-matrix` |
| Implement PID / state-space / transfer-function control | `tpt-eng-controls` |
| Compute loads, beam/frame analysis, code checks | `tpt-eng-structural` |
| Look up material properties (with data provenance) | `tpt-eng-materials` |
| Compute cross-section properties (rect/circle/I-section) | `tpt-eng-sections` |
| Apply code load cases / design factors / pass-fail checks | `tpt-eng-standards` |
| Roll up 1-D dimensional tolerance stack-ups (worst-case/RSS/Monte-Carlo) | `tpt-eng-tolerance` |
| Model GD&T zones, datum frames, material modifiers | `tpt-eng-gdt` |
| Compute fatigue life, Weibull reliability, run an FMEA | `tpt-eng-reliability` |
| Compute limit-state margins / utilization ratios | `tpt-eng-safety` |
| Work with points/vectors/frames in 2D/3D | `tpt-eng-geometry` |
| Build B-spline / NURBS curves and surfaces | `tpt-eng-nurbs` |
| Represent/repair/query a triangle mesh | `tpt-eng-mesh` |
| Build solids via SDF/CSG modeling | `tpt-eng-cad` |
| Read/write STL, OBJ, JSON, or CSV files | `tpt-eng-io` |
| Draw a chart or section diagram | `tpt-eng-plot` |
| Generate a calculation report (Markdown/HTML/JSON) | `tpt-eng-report` |
| Run any of the above from a command line, no Rust required | `tpt-eng-cli` |
| A full worked cross-crate scenario | `tpt-eng-examples` |
| Compute electrical impedance / per-unit / three-phase power | `tpt-eng-electrical` |
| Compute convection/radiation heat transfer and conduction | `tpt-eng-heat-transfer` |
| Look up general real-gas / VLE mixture properties | `tpt-eng-props-mixture` |
| Design a PCB stackup / trace / via | `tpt-eng-pcb` |
| Size a heat sink or solve a fan-curve operating point | `tpt-eng-thermal-mgmt` |
| Model transformers, generators, or transmission lines | `tpt-eng-power-components` |
| Model solar PV, wind turbines, or battery degradation | `tpt-eng-renewables` |
| Model tire forces, aero drag, or suspension kinematics | `tpt-eng-vehicle-dynamics` |
| Model hyperelastic tissue or implant geometry | `tpt-eng-biomech` |
| Size distillation columns / heat exchangers, pump curves | `tpt-eng-unit-ops` |
| Work with Miller indices, slip systems, crystal symmetry | `tpt-eng-crystallography` |
| Model soil behavior or borehole stratigraphy | `tpt-eng-geotech` |
| Compute HVAC loads, plumbing, or electrical panel schedules | `tpt-eng-building-sys` |
| Plan/schedule projects (CPM/PERT, Earned Value Management) | `tpt-eng-schedule` |
