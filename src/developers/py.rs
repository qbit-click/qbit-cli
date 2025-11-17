use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use anyhow::{bail, Context, Result};

use crate::utils::python::find_python;

/// Initialize Python project (requirements.txt + venv)
pub fn init() -> Result<()> {
    ensure_requirements()?;

    let Some(py) = find_python() else {
        eprintln!("Python not found.");
        eprintln!("Hint: install it via `qbit install python`");
        bail!("python interpreter not available");
    };
    println!("Using interpreter: {py}");

    ensure_venv(&py)?;

    println!("Done.");
    Ok(())
}

/// Install a dependency inside the managed venv and refresh requirements.txt.
pub fn add_package(package: &str) -> Result<()> {
    ensure_requirements()?;
    let interpreter = resolve_and_prepare_python()?;
    pip_install(&interpreter, package)?;
    refresh_requirements(&interpreter)?;
    println!("Package `{package}` installed and requirements.txt updated.");
    Ok(())
}

/// Remove a dependency inside the managed venv and refresh requirements.txt.
pub fn remove_package(package: &str) -> Result<()> {
    ensure_requirements()?;
    let interpreter = resolve_and_prepare_python()?;
    pip_remove(&interpreter, package)?;
    refresh_requirements(&interpreter)?;
    println!("Package `{package}` removed (if installed) and requirements.txt updated.");
    Ok(())
}

fn ensure_requirements() -> Result<()> {
    if !Path::new("requirements.txt").exists() {
        fs::write("requirements.txt", b"# pin your dependencies here\n")
            .context("writing requirements.txt")?;
        println!("Created requirements.txt");
    } else {
        println!("requirements.txt already exists");
    }
    Ok(())
}

fn ensure_venv(py: &str) -> Result<()> {
    if Path::new("venv").exists() {
        println!("venv already exists");
        return Ok(());
    }

    let (bin, mut args) = split_first(py);
    args.push("-m");
    args.push("venv");
    args.push("venv");

    println!("Creating venv...");
    let status = Command::new(bin)
        .args(&args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("spawning python to create venv")?;

    if !status.success() {
        bail!(format!(
            "failed to create venv (command: {} {})",
            bin,
            args.join(" ")
        ));
    }

    println!("venv created at ./venv");
    Ok(())
}

fn resolve_and_prepare_python() -> Result<PathBuf> {
    let Some(py) = find_python() else {
        eprintln!("Python not found.");
        eprintln!("Hint: install it via `qbit install python`");
        bail!("python interpreter not available");
    };
    ensure_venv(&py)?;
    let venv_python = venv_python_path();
    if !venv_python.exists() {
        bail!("expected virtualenv python at {}", venv_python.display());
    }
    Ok(venv_python)
}

fn venv_python_path() -> PathBuf {
    if cfg!(windows) {
        Path::new("venv").join("Scripts").join("python.exe")
    } else {
        Path::new("venv").join("bin").join("python")
    }
}

fn pip_install(python: &Path, package: &str) -> Result<()> {
    println!("Installing `{package}` via pip...");
    let status = Command::new(python)
        .args(["-m", "pip", "install", package])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("running pip install")?;

    if !status.success() {
        bail!("pip install failed for `{package}`");
    }
    Ok(())
}

fn pip_remove(python: &Path, package: &str) -> Result<()> {
    println!("Removing `{package}` via pip...");
    let status = Command::new(python)
        .args(["-m", "pip", "uninstall", "-y", package])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("running pip uninstall")?;

    if !status.success() {
        bail!("pip uninstall failed for `{package}`");
    }
    Ok(())
}

fn refresh_requirements(python: &Path) -> Result<()> {
    println!("Syncing requirements.txt via `pip freeze`...");
    let output = Command::new(python)
        .args(["-m", "pip", "freeze"])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .output()
        .context("running pip freeze")?;

    if !output.status.success() {
        bail!("pip freeze failed");
    }

    fs::write("requirements.txt", output.stdout).context("writing requirements.txt from freeze")?;
    Ok(())
}

/// Split "py -3" into ("py", ["-3"])
fn split_first(cmd: &str) -> (&str, Vec<&str>) {
    let mut parts = cmd.split_whitespace();
    let bin = parts.next().unwrap_or(cmd);
    let rest: Vec<&str> = parts.collect();
    (bin, rest)
}
