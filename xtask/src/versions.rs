//! Crate-version tracking: `cargo xtask versions` writes `docs/CRATE_VERSIONS.md`
//! from the manifests and the crates.io registry; `cargo xtask pin-check` runs
//! offline and fails when a workspace pin differs from its crate's version.

use std::collections::BTreeMap;
use std::process::Command;

use super::{parse_workspace_members, read_text, workspace_root, write_file};

/// User-Agent for registry requests. Project URL only, no personal contact.
const USER_AGENT: &str = "tpt-engineering-xtask (https://github.com/tpt-solutions/tpt-engineering)";

/// One publishable workspace crate and its version facts.
struct CrateVersion {
    name: String,
    local: String,
    pin: Option<String>,
    /// Newest stable version on crates.io; `None` if the crate is unpublished.
    registry: Option<String>,
    /// Every version listed on crates.io, newest first.
    published: Vec<String>,
}

/// Parse `major.minor.patch` into comparable numbers.
fn semver(v: &str) -> Option<(u64, u64, u64)> {
    let mut it = v.split('.').map(|p| p.parse::<u64>().ok());
    Some((it.next()??, it.next()??, it.next()??))
}

/// Compare two versions; `None` if either is not `x.y.z`.
fn compare(local: &str, registry: &str) -> Option<std::cmp::Ordering> {
    Some(semver(local)?.cmp(&semver(registry)?))
}

/// `[workspace.dependencies]` pins, keyed by crate name.
fn parse_pins(root_cargo: &str) -> BTreeMap<String, String> {
    let mut pins = BTreeMap::new();
    for line in root_cargo.lines() {
        let line = line.trim();
        if !line.starts_with("tpt-eng-") {
            continue;
        }
        let Some((name, rest)) = line.split_once(" = ") else {
            continue;
        };
        if let Some(v) = rest
            .split("version = \"")
            .nth(1)
            .and_then(|r| r.split('"').next())
        {
            pins.insert(name.trim().to_string(), v.to_string());
        }
    }
    pins
}

/// Read the registry entry for `name`. Returns `(newest_stable, all_versions)`;
/// `(None, _)` means the crate is not on crates.io.
fn registry_versions(name: &str) -> Result<(Option<String>, Vec<String>), String> {
    let url = format!("https://crates.io/api/v1/crates/{name}");
    let out = Command::new("curl")
        .args(["-sSL", "-w", "\n%{http_code}", "-A", USER_AGENT, &url])
        .output()
        .map_err(|e| format!("cannot run curl for {name}: {e}"))?;
    if !out.status.success() {
        return Err(format!("registry request failed for {name}"));
    }
    let text = String::from_utf8_lossy(&out.stdout).into_owned();
    let (body, code) = text.rsplit_once('\n').unwrap_or((&text, ""));
    match code.trim() {
        "404" => return Ok((None, Vec::new())),
        "200" => {}
        other => return Err(format!("registry returned HTTP {other} for {name}")),
    }
    let newest = body
        .split("\"max_stable_version\":\"")
        .nth(1)
        .and_then(|r| r.split('"').next())
        .map(str::to_string);
    let all: Vec<String> = body
        .split("\"num\":\"")
        .skip(1)
        .filter_map(|r| r.split('"').next().map(str::to_string))
        .collect();
    Ok((newest, all))
}

/// Civil date (UTC) from the system clock, as `YYYY-MM-DD`.
fn today() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    // Howard Hinnant's days-to-civil algorithm.
    let z = (secs / 86_400) as i64 + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + i64::from(m <= 2);
    format!("{y:04}-{m:02}-{d:02}")
}

/// Version string from a `version = "..."` line, or the workspace version when
/// the manifest inherits it.
fn package_version(cargo: &str, workspace_version: &str) -> String {
    if let Some(v) = cargo.lines().find_map(|l| {
        l.strip_prefix("version = \"")
            .map(|v| v.trim_end_matches('"').to_string())
    }) {
        return v;
    }
    if cargo.contains("version.workspace = true") {
        return workspace_version.to_string();
    }
    "?".to_string()
}

/// Load every publishable `tpt-eng-*` member with its local version and pin.
fn load_members() -> Result<Vec<CrateVersion>, String> {
    let root = workspace_root();
    let root_cargo = read_text(&root.join("Cargo.toml"))?;
    let ws_version = root_cargo
        .split("[workspace.package]")
        .nth(1)
        .and_then(|s| {
            s.lines().find_map(|l| {
                l.strip_prefix("version = \"")
                    .map(|v| v.trim_end_matches('"').to_string())
            })
        })
        .unwrap_or_else(|| "0.0.0".to_string());
    let pins = parse_pins(&root_cargo);
    let mut crates = Vec::new();
    for member in parse_workspace_members(&root_cargo) {
        let cargo = read_text(&root.join(&member).join("Cargo.toml"))?;
        if cargo.contains("publish = false") {
            continue;
        }
        let name = cargo
            .lines()
            .find_map(|l| l.strip_prefix("name = \""))
            .map(|n| n.trim_end_matches('"').to_string())
            .unwrap_or_default();
        if !name.starts_with("tpt-eng-") {
            continue;
        }
        crates.push(CrateVersion {
            local: package_version(&cargo, &ws_version),
            pin: pins.get(&name).cloned(),
            name,
            registry: None,
            published: Vec::new(),
        });
    }
    Ok(crates)
}

/// Offline check: every workspace pin must equal its crate's local version.
pub fn pin_check() -> Result<(), String> {
    let crates = load_members()?;
    let mismatches: Vec<String> = crates
        .iter()
        .filter_map(|c| match &c.pin {
            Some(p) if *p != c.local => Some(format!(
                "`{}` pinned at {p} but its version is {}",
                c.name, c.local
            )),
            _ => None,
        })
        .collect();
    if mismatches.is_empty() {
        println!("pin-check: all workspace pins match their crate versions");
        Ok(())
    } else {
        Err(format!("pin-check failed:\n  {}", mismatches.join("\n  ")))
    }
}

/// Fetch registry data and write `docs/CRATE_VERSIONS.md`. With `offline`, the
/// registry columns are left as "not fetched".
pub fn write_report(offline: bool) -> Result<(), String> {
    let mut crates = load_members()?;
    if !offline {
        for c in &mut crates {
            let (newest, all) = registry_versions(&c.name)?;
            c.registry = newest;
            c.published = all;
        }
    }
    let (mut in_sync, mut ahead, mut behind, mut unpublished) = (0, 0, 0, 0);
    let mut rows = String::new();
    for c in &crates {
        let state = if offline {
            "not fetched".to_string()
        } else {
            match &c.registry {
                None => {
                    unpublished += 1;
                    "unpublished".to_string()
                }
                Some(r) => match compare(&c.local, r) {
                    Some(std::cmp::Ordering::Equal) => {
                        in_sync += 1;
                        "in sync".to_string()
                    }
                    Some(std::cmp::Ordering::Greater) => {
                        ahead += 1;
                        "local ahead (unreleased)".to_string()
                    }
                    _ => {
                        behind += 1;
                        "registry ahead (check)".to_string()
                    }
                },
            }
        };
        let registry = match (offline, &c.registry) {
            (true, _) | (_, None) => "-".to_string(),
            (_, Some(r)) => r.clone(),
        };
        let pin = c.pin.clone().unwrap_or_else(|| "-".to_string());
        let history = if c.published.is_empty() {
            "-".to_string()
        } else {
            c.published.join(", ")
        };
        rows.push_str(&format!(
            "| `{}` | {} | {} | {} | {} | {} |\n",
            c.name, c.local, pin, registry, state, history
        ));
    }
    let summary = if offline {
        format!(
            "{} publishable crates (registry not fetched).",
            crates.len()
        )
    } else {
        format!(
            "{} publishable crates: {in_sync} in sync, {ahead} local ahead of crates.io, \
             {behind} registry ahead, {unpublished} unpublished.",
            crates.len()
        )
    };
    let doc = format!(
        "# Crate versions\n\n\
         Tracking file for the latest version of every publishable `tpt-eng-*` crate.\n\
         Generated by `cargo xtask versions` on {}. Do not edit by hand.\n\n\
         - **Local** is the version in the crate's `Cargo.toml`, or the workspace\n\
           version when inherited.\n\
         - **Pin** is the version in the root `[workspace.dependencies]` table. It must\n\
           equal **Local** (`cargo xtask pin-check`, run by `cargo xtask check`).\n\
         - **crates.io** is the newest stable version on the registry. **State**\n\
           compares **Local** with it.\n\
         - **Published versions** lists every release on the registry, newest first.\n\n\
         {summary}\n\n\
         | Crate | Local | Pin | crates.io | State | Published versions |\n\
         |---|---|---|---|---|---|\n\
         {rows}",
        today()
    );
    let path = workspace_root().join("docs").join("CRATE_VERSIONS.md");
    write_file(&path, &doc)?;
    println!("wrote {}", path.display());
    println!("{summary}");
    Ok(())
}
