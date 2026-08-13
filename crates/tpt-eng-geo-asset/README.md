# tpt-eng-geo-asset

Geographic asset registry: maps geographic coordinates to logical device /
network nodes. Supports id lookup and spatial queries (nearest asset, assets
within a radius) using the haversine great-circle distance.

## Example

```rust
use tpt_eng_geo_asset::{Asset, AssetKind, AssetRegistry, GeoCoord};

let mut r = AssetRegistry::new();
r.register(Asset::new("A", AssetKind::Sensor, GeoCoord::new(0.0, 0.0), "n1"));
assert_eq!(r.get("A").unwrap().logical_node, "n1");
assert_eq!(r.nearest(GeoCoord::new(0.1, 0.1)).unwrap().id, "A");
```

Dual-licensed under MIT OR Apache-2.0. Copyright (c) 2026 TPT Solutions.
