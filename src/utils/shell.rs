use anyhow::{Context, Result, bail};
use std::process::{Command, Stdio};

pub fn run_shell(command: &str) -> Result<()> {
    let mut cmd = shell_command(command);
    cmd.stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let status = cmd
        .status()
        .with_context(|| format!("running shell command: {command}"))?;

    if !status.success() {
        bail!(
            "command `{}` exited with code {}",
            command,
            status.code().unwrap_or_default()
        );
    }
    Ok(())
}

pub fn run_commands(label: &str, commands: &[String]) -> Result<()> {
    if commands.is_empty() {
        bail!("no commands defined for {label}");
    }

    for (idx, cmd) in commands.iter().enumerate() {
        println!("[{label}] step {} -> {}", idx + 1, cmd);
        run_shell(cmd)?;
    }

    Ok(())
}

#[cfg(windows)]
fn shell_command(command: &str) -> Command {
    let mut cmd = Command::new("cmd");
    cmd.arg("/C");
    cmd.arg(command);
    cmd
}

#[cfg(not(windows))]
fn shell_command(command: &str) -> Command {
    let mut cmd = Command::new("sh");
    cmd.arg("-c");
    cmd.arg(command);
    cmd
}
