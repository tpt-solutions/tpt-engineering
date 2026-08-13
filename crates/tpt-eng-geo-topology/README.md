# tpt-eng-geo-topology

Directional infrastructure graphs for pipes / wires / ducts, with
upstream/downstream traversal and flow-direction logic.

## Example

```rust
use tpt_eng_geo_topology::{Edge, EdgeKind, Topology};

let mut topo = Topology::new();
topo.add_node("tank");
topo.add_node("pump");
topo.add_node("valve");
topo.add_edge(Edge::new("e1", "tank", "pump", EdgeKind::Pipe, 1.0));
topo.add_edge(Edge::new("e2", "pump", "valve", EdgeKind::Pipe, 1.0));

let mut down = topo.downstream("tank");
down.sort();
assert_eq!(down, vec!["pump", "valve"]);
```

Dual-licensed under MIT OR Apache-2.0. Copyright (c) 2026 TPT Solutions.
