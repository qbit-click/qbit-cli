use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

use crate::utils::python::find_python;

pub fn cmd_py_init() -> anyhow::Result<i32> {
    // 1) ensure requirements.txt
    if !Path::new("requirements.txt").exists() {
        fs::write("requirements.txt", b"# pinned deps go here\n")?;
        println!("📄 requirements.txt created");
    } else {
        println!("ℹ️ requirements.txt already exists");
    }

    // 2) find python
    let Some(py) = find_python() else {
        eprintln!("❌ Python not found.");
        eprintln!("➡️ Install it via: qbit install python");
        return Ok(1);
    };
    println!("🐍 Using interpreter: {py}");

    // 3) create venv if not exists
    if !Path::new("venv").exists() {
        let (bin, mut args) = split_first(&py);
        args.push("-m");
        args.push("venv");
        args.push("venv");
        let status = Command::new(bin)
            .args(&args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?;

        if !status.success() {
            eprintln!("❌ Failed to create venv (command: {} {}).", bin, args.join(" "));
            eprintln!("➡️ Try installing Python: qbit install python");
            return Ok(status.code().unwrap_or(1));
        }
        println!("✅ venv created at ./venv");
    } else {
        println!("ℹ️ venv already exists");
    }

    Ok(0)
}

// small local helper to mirror utils split
fn split_first(cmd: &str) -> (&str, Vec<&str>) {
    let mut parts = cmd.split_whitespace();
    let bin = parts.next().unwrap_or(cmd);
    let mut rest: Vec<&str> = parts.collect();
    (bin, rest)
}
