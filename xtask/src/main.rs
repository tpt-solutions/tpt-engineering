//! `xtask` — developer tooling for the `tpt-engineering` workspace.
//!
//! One-stop `cargo xtask <cmd>` entry points used both locally and in CI.
//! Every command shells out to `cargo` (and `cargo-deny` where relevant) so
//! the behaviour is transparent and easy to reproduce by hand.
//!
//! Commands:
//! * `new-crate <tpt-eng-name>` — scaffold a new `tpt-eng-*` crate and register
//!   it in the workspace (`members` + `[workspace.dependencies]`) and the README
//!   inventory. It generates `Cargo.toml`, `src/lib.rs`, `README.md`,
//!   `tests/basic.rs`, and `examples/basic.rs` (matching the
//!   `tpt-eng-examples` runnable-example pattern). Add `--dry-run` to print the
//!   generated files instead of writing.
//! * `no-std-matrix` — build every `no_std`-capable crate for
//!   `thumbv6m-none-eabi` (replaces the hand-listed CI steps; see ADR 0001).
//! * `fmt` / `clippy` / `test` / `deny` / `check` — the standard hygiene gates.
//! * `doctest` / `doc` — run documentation tests, or build the docs.
//!
//! This crate is intentionally dependency-free (pure `std`) so it always builds
//! offline and adds nothing to the supply-chain surface of the engineering crates.

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// `no_std`-capable crates, mirroring the CI `no_std` job and the
/// `no_std = true` entries for this repo in `tpt-rust-map/registry.toml`.
///
/// NOTE: `tpt-eng-props` (the umbrella re-export) is intentionally excluded.
/// It depends on `tpt-math-units`, which unconditionally pulls `uom` and
/// `num-traits`. Building it `--no-default-features` still activates the leaf
/// crates' default `std` features via feature unification, enabling
/// `num-traits/std`, which is unavailable on `thumbv6m-none-eabi`. Only the
/// leaf fluid-property crates below build cleanly no_std.
const NO_STD_CRATES: &[&str] = &[
    "tpt-eng-props-water",
    "tpt-eng-props-air",
    "tpt-eng-props-fuels",
];

const NO_STD_TARGET: &str = "thumbv6m-none-eabi";

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if let Err(err) = run(&args) {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}

fn run(args: &[String]) -> Result<(), String> {
    match args.split_first() {
        None => {
            print_usage();
        }
        Some((cmd, rest)) => match cmd.as_str() {
            "fmt" => cargo(&["fmt", "--all"]),
            "clippy" => cargo(&[
                "clippy",
                "--workspace",
                "--all-targets",
                "--all-features",
                "--",
                "-D",
                "warnings",
            ]),
            "test" => cargo(&["test", "--workspace", "--all-features"]),
            "doctest" => cargo(&["test", "--doc", "--workspace", "--all-features"]),
            "doc" => cargo(&["doc", "--workspace", "--no-deps", "--all-features"]),
            "deny" => cargo_deny(&["check"]),
            "no-std-matrix" => no_std_matrix(),
            "check" => {
                cargo(&["fmt", "--all", "--", "--check"])?;
                cargo(&[
                    "clippy",
                    "--workspace",
                    "--all-targets",
                    "--all-features",
                    "--",
                    "-D",
                    "warnings",
                ])?;
                cargo_deny(&["check"])?;
                Ok(())
            }
            "new-crate" => new_crate(rest),
            other => Err(format!(
                "unknown command `{other}`; run `cargo xtask` with no args for usage"
            )),
        }?,
    }
    Ok(())
}

fn print_usage() {
    println!(
        "usage: cargo xtask <command> [args]\n\
         \n\
         commands:\n\
           new-crate <tpt-eng-name> [--desc \"...\"] [--domain \"...\"] [--no-std yes|no] [--dry-run]\n\
           \x20\x20\x20\x20scaffolds Cargo.toml, src/lib.rs, README.md, tests/basic.rs, examples/basic.rs\n\
           no-std-matrix   build the no_std crates for {NO_STD_TARGET}\n\
           fmt | clippy | test | doctest | doc | deny | check   workspace hygiene gates"
    );
}

// ---------------------------------------------------------------------------
// no_std matrix
// ---------------------------------------------------------------------------

fn no_std_matrix() -> Result<(), String> {
    // Best-effort: make sure the bare-metal target is installed locally.
    let _ = Command::new("rustup")
        .args(["target", "add", NO_STD_TARGET])
        .status();
    for name in NO_STD_CRATES {
        let status = Command::new("cargo")
            .args([
                "build",
                "-p",
                name,
                "--no-default-features",
                "--target",
                NO_STD_TARGET,
            ])
            .status()
            .map_err(|e| format!("failed to spawn cargo: {e}"))?;
        if !status.success() {
            return Err(format!("no_std build of `{name}` failed"));
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// new-crate scaffolding
// ---------------------------------------------------------------------------

fn new_crate(args: &[String]) -> Result<(), String> {
    let mut name: Option<String> = None;
    let mut desc = String::new();
    let mut domain = String::from("general");
    let mut no_std = String::from("yes");
    let mut dry_run = false;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--desc" => desc = next_arg(args, &mut i, "--desc")?,
            "--domain" => domain = next_arg(args, &mut i, "--domain")?,
            "--no-std" => no_std = next_arg(args, &mut i, "--no-std")?,
            "--dry-run" => dry_run = true,
            s if !s.starts_with("--") && name.is_none() => name = Some(s.to_string()),
            s => return Err(format!("unexpected argument `{s}`")),
        }
        i += 1;
    }

    let name = name.ok_or(
        "usage: xtask new-crate <tpt-eng-name> [--desc \"...\"] [--domain \"...\"] [--no-std yes|no] [--dry-run]",
    )?;
    if !name.starts_with("tpt-eng-") {
        return Err(format!(
            "crate name must start with `tpt-eng-` (got `{name}`)"
        ));
    }
    if desc.is_empty() {
        desc = format!("TODO: describe {name}");
    }

    let root = workspace_root();
    let crate_dir = root.join("crates").join(&name);
    if crate_dir.exists() {
        return Err(format!("crates/{name} already exists"));
    }

    let cargo_toml = render_crate_cargo_toml(&name, &desc);
    let lib_rs = render_crate_lib_rs(&name, &desc);
    let readme = render_crate_readme(&name, &desc);
    let tests_basic = render_crate_tests_basic();
    let example_basic = render_crate_example_basic(&name);

    let root_cargo_path = root.join("Cargo.toml");
    let root_readme_path = root.join("README.md");
    let root_cargo = fs::read_to_string(&root_cargo_path)
        .map_err(|e| format!("read {}: {e}", root_cargo_path.display()))?;
    let root_readme = fs::read_to_string(&root_readme_path)
        .map_err(|e| format!("read {}: {e}", root_readme_path.display()))?;

    let new_root_cargo = register_member(&root_cargo, &name)?;
    let new_root_cargo = register_workspace_dep(&new_root_cargo, &name)?;
    let new_root_readme = register_readme_row(&root_readme, &name, &domain, &no_std, &desc)?;

    if dry_run {
        println!("=== would write crates/{name}/Cargo.toml ===\n{cargo_toml}");
        println!("=== would write crates/{name}/src/lib.rs ===\n{lib_rs}");
        println!("=== would write crates/{name}/README.md ===\n{readme}");
        println!("=== would write crates/{name}/tests/basic.rs ===\n{tests_basic}");
        println!("=== would write crates/{name}/examples/basic.rs ===\n{example_basic}");
        println!("=== Cargo.toml: member + workspace dependency added ===");
        println!("=== README.md: inventory row added ===");
        return Ok(());
    }

    fs::create_dir_all(crate_dir.join("src")).map_err(|e| format!("create crates/{name}: {e}"))?;
    fs::create_dir_all(crate_dir.join("tests"))
        .map_err(|e| format!("create crates/{name}/tests: {e}"))?;
    fs::create_dir_all(crate_dir.join("examples"))
        .map_err(|e| format!("create crates/{name}/examples: {e}"))?;
    write_file(&crate_dir.join("Cargo.toml"), &cargo_toml)?;
    write_file(&crate_dir.join("src").join("lib.rs"), &lib_rs)?;
    write_file(&crate_dir.join("README.md"), &readme)?;
    write_file(&crate_dir.join("tests").join("basic.rs"), &tests_basic)?;
    write_file(&crate_dir.join("examples").join("basic.rs"), &example_basic)?;
    write_file(&root_cargo_path, &new_root_cargo)?;
    write_file(&root_readme_path, &new_root_readme)?;

    println!("created crate `{name}`.");
    println!(
        "next: add dependencies to crates/{name}/Cargo.toml, implement src/lib.rs,\n\
         then run `cargo test -p {name}` and `cargo run -p {name} --example basic`,\n\
         and finally `cargo xtask check`."
    );
    Ok(())
}

fn next_arg(args: &[String], i: &mut usize, flag: &str) -> Result<String, String> {
    let next = args
        .get(*i + 1)
        .ok_or(format!("`{flag}` requires a value"))?;
    if next.starts_with("--") {
        return Err(format!("`{flag}` requires a value"));
    }
    *i += 1;
    Ok(next.clone())
}

/// Register `crates/<name>` in the workspace `members` list (before its `]`).
fn register_member(root: &str, name: &str) -> Result<String, String> {
    let lines: Vec<&str> = root.lines().collect();
    let start = lines
        .iter()
        .position(|l| l.trim() == "members = [")
        .ok_or("could not find `members = [` in workspace Cargo.toml")?;
    let end = (start + 1..lines.len())
        .find(|&j| lines[j].trim() == "]")
        .ok_or("could not find closing `]` of members")?;
    let mut out = String::new();
    for (idx, l) in lines.iter().enumerate() {
        out.push_str(l);
        out.push('\n');
        if idx == end - 1 {
            out.push_str(&format!("    \"crates/{name}\",\n"));
        }
    }
    Ok(out)
}

/// Register `name = { version = "0.1.0", path = "crates/<name>" }` in
/// `[workspace.dependencies]`, before the next section header.
fn register_workspace_dep(root: &str, name: &str) -> Result<String, String> {
    let lines: Vec<&str> = root.lines().collect();
    let h = lines
        .iter()
        .position(|l| l.trim() == "[workspace.dependencies]")
        .ok_or("could not find `[workspace.dependencies]` in Cargo.toml")?;
    let next_section = (h + 1..lines.len())
        .find(|&j| lines[j].trim_start().starts_with('['))
        .unwrap_or(lines.len());
    let mut out = String::new();
    for (idx, l) in lines.iter().enumerate() {
        out.push_str(l);
        out.push('\n');
        if idx + 1 == next_section {
            out.push_str(&format!(
                "{name} = {{ version = \"0.1.0\", path = \"crates/{name}\" }}\n"
            ));
        }
    }
    Ok(out)
}

/// Append a row to the README crate-inventory table.
fn register_readme_row(
    readme: &str,
    name: &str,
    domain: &str,
    no_std: &str,
    desc: &str,
) -> Result<String, String> {
    let lines: Vec<&str> = readme.lines().collect();
    let last = lines
        .iter()
        .rposition(|l| l.trim_start().starts_with("| ") && l.contains("`tpt-eng-"))
        .ok_or("could not find the crate inventory table in README.md")?;
    let mut out = String::new();
    for (idx, l) in lines.iter().enumerate() {
        out.push_str(l);
        out.push('\n');
        if idx == last {
            out.push_str(&format!("| `{name}` | {domain} | {no_std} | {desc} |\n"));
        }
    }
    Ok(out)
}

fn render_crate_cargo_toml(name: &str, desc: &str) -> String {
    format!(
        "[package]\n\
         name = \"{name}\"\n\
         version = \"0.1.0\"\n\
         edition.workspace = true\n\
         license.workspace = true\n\
         authors.workspace = true\n\
         description = \"{desc}\"\n\
         readme = \"README.md\"\n\
         repository.workspace = true\n\
         homepage.workspace = true\n\
         \n\
         [dependencies]\n\
         tpt-math-units = {{ workspace = true }}\n\
         \n\
         [features]\n\
         default = [\"std\"]\n\
         std = [\"tpt-math-units/std\"]\n\
         alloc = [\"tpt-math-units/alloc\"]\n\
         \n\
         [lints]\n\
         workspace = true\n"
    )
}

fn render_crate_lib_rs(name: &str, desc: &str) -> String {
    format!(
        "//! # {name}\n\
         //!\n\
         //! {desc}.\n\
         //!\n\
         //! Scaffolded by `cargo xtask new-crate`. Implement the crate's\n\
         //! primitives here, following the pattern of the sibling `tpt-eng-*`\n\
         //! crates.\n\
         \n\
         #![cfg_attr(not(feature = \"std\"), no_std)]\n"
    )
}

fn render_crate_readme(name: &str, desc: &str) -> String {
    format!(
        "# {name}\n\
         \n\
         {desc}.\n\
         \n\
         Scaffolded by `cargo xtask new-crate`. See the [workspace\n\
         README](https://github.com/tpt-solutions/tpt-engineering) for conventions.\n\
         \n\
         Dual-licensed under MIT OR Apache-2.0.\n"
    )
}

fn render_crate_tests_basic() -> String {
    "\
// Basic smoke test for a newly scaffolded `tpt-eng-*` crate.
//
// Scaffolded by `cargo xtask new-crate`. Expand with real unit/integration
// tests as the crate's public API lands.

#[test]
fn basic() {
    assert!(true);
}
"
    .to_string()
}

fn render_crate_example_basic(name: &str) -> String {
    format!(
        "// Basic runnable example for `{name}`.\n\
         //\n\
         // Scaffolded by `cargo xtask new-crate`. Implement the crate's\n\
         // primitives in `src/lib.rs`, then extend this example to demonstrate\n\
         // them.\n\
         \n\
         fn main() {{\n\
         \x20\x20\x20\x20println!(\"{name} scaffold example — implement src/lib.rs\");\n\
         }}\n"
    )
}

// ---------------------------------------------------------------------------
// command helpers
// ---------------------------------------------------------------------------

fn cargo(args: &[&str]) -> Result<(), String> {
    let status = Command::new("cargo")
        .args(args)
        .status()
        .map_err(|e| format!("failed to spawn cargo: {e}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("`cargo {}` failed", args.join(" ")))
    }
}

fn cargo_deny(args: &[&str]) -> Result<(), String> {
    let has_standalone = which("cargo-deny");
    let mut cmd = Command::new(if has_standalone {
        "cargo-deny"
    } else {
        "cargo"
    });
    if !has_standalone {
        cmd.arg("deny");
    }
    cmd.args(args);
    let status = cmd
        .status()
        .map_err(|e| format!("failed to spawn cargo-deny: {e}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("`cargo deny {}` failed", args.join(" ")))
    }
}

fn which(name: &str) -> bool {
    Command::new(if cfg!(windows) { "where" } else { "which" })
        .arg(name)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn workspace_root() -> PathBuf {
    env::current_dir().expect("current working directory is available")
}

fn write_file(path: &std::path::Path, contents: &str) -> Result<(), String> {
    fs::write(path, contents).map_err(|e| format!("write {}: {e}", path.display()))
}
