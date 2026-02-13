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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shell_command_uses_platform_shell() {
        let cmd = shell_command("echo hi");
        let program = cmd.get_program().to_string_lossy().to_string();
        let args: Vec<String> = cmd
            .get_args()
            .map(|arg| arg.to_string_lossy().to_string())
            .collect();

        #[cfg(windows)]
        {
            let lower = program.to_ascii_lowercase();
            assert!(lower == "cmd" || lower.ends_with("cmd.exe"));
            assert!(args.iter().any(|arg| arg.eq_ignore_ascii_case("/C")));
            assert!(args.iter().any(|arg| arg == "echo hi"));
        }

        #[cfg(not(windows))]
        {
            assert_eq!(program, "sh");
            assert!(args.iter().any(|arg| arg == "-c"));
            assert!(args.iter().any(|arg| arg == "echo hi"));
        }
    }

    #[test]
    fn run_commands_rejects_empty_command_list() {
        let err = run_commands("demo", &[]).expect_err("must fail");
        assert!(err.to_string().contains("no commands defined"));
    }
}
