//! Integration tests for the `tpt-eng-cli` binary.

use std::io::Write;
use std::process::Command;

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_tpt-eng-cli"))
}

#[test]
fn test_units_convert() {
    let out = bin().args(["units", "convert", "1", "m", "mm"]).output().unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8(out.stdout).unwrap();
    assert!(stdout.contains("1000 mm"), "got: {stdout}");
}

#[test]
fn test_units_convert_temperature() {
    let out = bin().args(["units", "convert", "100", "C", "F"]).output().unwrap();
    assert!(out.status.success());
    assert!(String::from_utf8(out.stdout).unwrap().contains("212"));
}

#[test]
fn test_units_incompatible() {
    let out = bin().args(["units", "convert", "1", "m", "N"]).output().unwrap();
    assert!(!out.status.success());
}

#[test]
fn test_materials_inspect() {
    let out = bin().args(["materials", "steel"]).output().unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8(out.stdout).unwrap();
    assert!(stdout.contains("Young's modulus"));
}

#[test]
fn test_sections_inspect() {
    let out = bin()
        .args(["sections", "inspect", "rectangle", "0.1", "0.2"])
        .output()
        .unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8(out.stdout).unwrap();
    assert!(stdout.contains("area:"));
}

#[test]
fn test_sections_ibeam() {
    let out = bin()
        .args(["sections", "inspect", "i-beam", "0.3", "0.15", "0.01", "0.006"])
        .output()
        .unwrap();
    assert!(out.status.success());
}

#[test]
fn test_calc_beam() {
    let out = bin()
        .args([
            "calc", "beam", "5", "10000", "--material", "steel", "rectangle", "0.1", "0.2",
        ])
        .output()
        .unwrap();
    assert!(out.status.success(), "{:?}", String::from_utf8(out.stderr));
    let stdout = String::from_utf8(out.stdout).unwrap();
    assert!(stdout.contains("M_max"));
}

#[test]
fn test_report_generates_all_formats() {
    let dir = std::env::temp_dir();
    for ext in ["md", "html", "json"] {
        let out_path = dir.join(format!("tpt_cli_report_test.{ext}"));
        let out = bin()
            .args(["report", "--out", &out_path.to_string_lossy()])
            .output()
            .unwrap();
        assert!(out.status.success(), "{ext}: {:?}", String::from_utf8(out.stderr));
        assert!(out_path.exists());
        let _ = std::fs::remove_file(&out_path);
    }
}

#[test]
fn test_validate_json() {
    let dir = std::env::temp_dir();
    let path = dir.join("tpt_cli_validate_test.json");
    {
        let mut f = std::fs::File::create(&path).unwrap();
        f.write_all(b"{\"name\":\"test\",\"value\":1.0}").unwrap();
    }
    let out = bin().args(["validate", &path.to_string_lossy()]).output().unwrap();
    assert!(out.status.success(), "{:?}", String::from_utf8(out.stderr));
    let _ = std::fs::remove_file(&path);
}

#[test]
fn test_validate_unknown_type_fails() {
    let dir = std::env::temp_dir();
    let path = dir.join("tpt_cli_validate_test.xyz");
    {
        let mut f = std::fs::File::create(&path).unwrap();
        f.write_all(b"nope").unwrap();
    }
    let out = bin().args(["validate", &path.to_string_lossy()]).output().unwrap();
    assert!(!out.status.success());
    let _ = std::fs::remove_file(&path);
}
