# tpt-eng-network-matrix

Automated generation of network matrices from an infrastructure
[`Topology`](tpt_eng_geo_topology::Topology) graph: the reduced
incidence matrix `A` and the nodal admittance (Laplacian) matrix `Y`, returned
as in-house [`DMatrix`](tpt_math_linalg::tpt_math_linalg_dense::DMatrix) from
[`tpt-math-linalg`](tpt_math_linalg).

## Example

```rust
use tpt_eng_geo_topology::{Edge, EdgeKind, Topology};
use tpt_eng_network_matrix::admittance_matrix;
use tpt_math_linalg::tpt_math_linalg_dense::DMatrix;

let mut t = Topology::new();
for n in ["A", "B", "C"] { t.add_node(n); }
t.add_edge(Edge::new("e1", "A", "B", EdgeKind::Wire, 1.0));
t.add_edge(Edge::new("e2", "B", "C", EdgeKind::Wire, 1.0));
t.add_edge(Edge::new("e3", "C", "A", EdgeKind::Wire, 1.0));

let y = admittance_matrix(&t);
assert_eq!(y[(0, 0)], 2.0); // each node touches two unit branches
assert_eq!(y[(0, 1)], -1.0);
```

Dual-licensed under MIT OR Apache-2.0. Copyright (c) 2026 TPT Solutions.
