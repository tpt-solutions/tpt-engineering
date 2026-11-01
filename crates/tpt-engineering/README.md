Umbrella meta-crate re-exporting every `tpt-eng-*` crate behind its own
feature flag. **This crate is not published to crates.io.** Depend on it by
path or git from this workspace, or depend on the individual `tpt-eng-*`
crates from crates.io directly (each is published at `0.1.0`).

Each feature is named after the crate without the `tpt-eng-` prefix (hyphens
become underscores in the module name):

```toml
[dependencies]
tpt-engineering = { path = "path/to/tpt-engineering/crates/tpt-engineering", features = ["structural", "props"] }
```

```rust,ignore
use tpt_engineering::structural;
use tpt_engineering::props;
```

Dual-licensed under MIT OR Apache-2.0.
