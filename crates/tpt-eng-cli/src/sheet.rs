//! `tpt-eng sheet`: run a declarative calculation sheet, or print its schema.

use anyhow::{Context, Result, bail};
use clap::{Args, Subcommand};
use std::path::PathBuf;

use tpt_eng_sheet::{Sheet, json_schema, run};

#[derive(Args)]
pub struct SheetArgs {
    #[command(subcommand)]
    cmd: SheetCmd,
}

#[derive(Subcommand)]
enum SheetCmd {
    /// Run a sheet file (.toml or .json) and produce its report.
    Run {
        /// Path to the sheet file.
        path: PathBuf,
        /// Write the report to this path (.md, .html or .json). Without it, the
        /// report is printed as Markdown, or as JSON with `--json`.
        #[arg(long)]
        out: Option<PathBuf>,
    },
    /// Print the JSON Schema that sheet files must satisfy.
    Schema {
        /// Write the schema to this path instead of printing it.
        #[arg(long)]
        out: Option<PathBuf>,
    },
}

pub fn run_sheet(args: SheetArgs, json: bool) -> Result<()> {
    match args.cmd {
        SheetCmd::Run { path, out } => run_file(&path, out.as_ref(), json),
        SheetCmd::Schema { out } => print_schema(out.as_ref()),
    }
}

fn run_file(path: &PathBuf, out: Option<&PathBuf>, json: bool) -> Result<()> {
    let text =
        std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let sheet = match path.extension().and_then(|e| e.to_str()) {
        Some("toml") => Sheet::from_toml(&text)?,
        Some("json") => Sheet::from_json(&text)?,
        _ => bail!("sheet files must end in .toml or .json: {}", path.display()),
    };
    let report = run(&sheet)?;
    match out {
        Some(path) => crate::write_report(&report, path)?,
        None if json => println!("{}", tpt_eng_report::to_json(&report)?),
        None => print!("{}", tpt_eng_report::to_markdown(&report)),
    }
    Ok(())
}

fn print_schema(out: Option<&PathBuf>) -> Result<()> {
    let schema = json_schema()?;
    match out {
        Some(path) => {
            std::fs::write(path, &schema).with_context(|| format!("writing {}", path.display()))?;
            println!("wrote schema {}", path.display());
        }
        None => print!("{schema}"),
    }
    Ok(())
}
