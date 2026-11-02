Umbrella meta-crate re-exporting every `tpt-eng-*` crate behind its own
feature flag. Published to crates.io as `0.1.0` (Batch 11). You can depend on
it from crates.io, or depend on the individual `tpt-eng-*` crates directly.

Each feature is named after the crate without the `tpt-eng-` prefix (hyphens
become underscores in the module name):

```toml
[dependencies]
tpt-engineering = { version = "0.1", features = ["structural", "props"] }
```

```rust,ignore
use tpt_engineering::structural;
use tpt_engineering::props;
```

Dual-licensed under MIT OR Apache-2.0.
