use std::process::{Command, Stdio};

/// Candidate interpreters to try (ordered).
#[cfg(windows)]
const CANDIDATES: &[&str] = &["py -3", "py", "python", "python3"];

#[cfg(not(windows))]
const CANDIDATES: &[&str] = &["python3", "python"];

/// Try to resolve a Python interpreter that responds to `--version`.
pub fn find_python() -> Option<String> {
    // Respect an explicit override if provided.
    if let Ok(explicit) = std::env::var("QBIT_PY") {
        if check_version_ok(&explicit) {
            return Some(explicit);
        }
    }
    for cand in CANDIDATES {
        if check_version_ok(cand) {
            return Some((*cand).to_string());
        }
    }
    None
}

fn check_version_ok(cmd: &str) -> bool {
    match run_status(cmd, &["--version"]) {
        Ok(st) => st.success(),
        Err(_) => false,
    }
}

fn run_status(cmd: &str, args: &[&str]) -> std::io::Result<std::process::ExitStatus> {
    let (bin, rest) = split_first(cmd);
    Command::new(bin)
        .args(rest)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
}

/// Split "py -3" into ("py", ["-3"])
fn split_first(cmd: &str) -> (&str, Vec<&str>) {
    let mut parts = cmd.split_whitespace();
    let bin = parts.next().unwrap_or(cmd);
    let rest: Vec<&str> = parts.collect();
    (bin, rest)
}
