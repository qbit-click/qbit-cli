use std::process::{Command, Stdio};

use anyhow::{Context, Result, bail};

use crate::config::{InstallSpec, load_project_config};
use crate::os::package_manager::{InstallCommand, PackageManager, detect_package_manager};

/// Entry point from CLI.
pub fn install_target(raw_spec: &str, dry_run: bool, yes: bool) -> Result<()> {
    let (logical_target, inline_version) = parse_target_spec(raw_spec)?;
    let selected_manager = detect_package_manager()?;

    let mut configured_version: Option<String> = None;
    let mut identifier = logical_target.clone();

    if let Some(cfg) = load_project_config()? {
        if let Some((entry_name, spec)) = cfg.install_target_case_insensitive(&logical_target) {
            configured_version = spec.configured_version().map(|version| version.to_string());
            identifier = resolve_identifier(spec, selected_manager.as_ref(), &logical_target);
            println!(
                "Using install config `{}` from {}",
                entry_name,
                cfg.path.display()
            );
        }
    }

    let inline_was_provided = inline_version.is_some();
    let requested_version = inline_version.or(configured_version.clone());

    if inline_was_provided && configured_version.is_some() {
        println!("Inline version override applied.");
    }

    println!("Selected package manager: {}", selected_manager.name());
    println!("Resolved identifier: {identifier}");
    if let Some(version) = requested_version.as_deref() {
        println!("Resolved version: {version}");
    } else {
        println!("Resolved version: latest available from package manager");
    }

    let mut install_cmd =
        selected_manager.build_install_cmd(&identifier, requested_version.as_deref())?;
    if yes {
        selected_manager.apply_yes_flag(&mut install_cmd);
    }

    execute_or_print_dry_run(&install_cmd, dry_run, execute_install)
}

fn parse_target_spec(spec: &str) -> Result<(String, Option<String>)> {
    let trimmed = spec.trim();
    if trimmed.is_empty() {
        bail!("Install target is empty. Use `qbit install <name[:version]>`.");
    }

    if let Some((name, version)) = trimmed.split_once(':') {
        let logical_name = name.trim();
        let requested_version = version.trim();
        if logical_name.is_empty() {
            bail!("Install target name is empty before `:`. Use `qbit install <name[:version]>`.");
        }
        if requested_version.is_empty() {
            bail!(
                "Version after `:` is empty. Use `qbit install {logical_name}` or provide a version."
            );
        }
        return Ok((
            logical_name.to_string(),
            Some(requested_version.to_string()),
        ));
    }

    Ok((trimmed.to_string(), None))
}

fn resolve_identifier(
    spec: &InstallSpec,
    manager: &dyn PackageManager,
    logical_target: &str,
) -> String {
    if let Some(identifier) = spec.global_identifier() {
        return identifier.to_string();
    }

    for key in manager.config_keys() {
        if let Some(identifier) = spec.identifier(key) {
            return identifier.to_string();
        }
    }

    if let Some(default_identifier) = spec.identifier("default") {
        return default_identifier.to_string();
    }

    logical_target.to_string()
}

fn execute_install(command: &InstallCommand) -> Result<()> {
    let status = Command::new(&command.program)
        .args(&command.args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .with_context(|| format!("running installer command: {}", command.render()))?;

    if !status.success() {
        bail!(
            "Installer command failed (exit code {}): {}",
            status.code().unwrap_or(1),
            command.render()
        );
    }

    Ok(())
}

fn execute_or_print_dry_run<F>(command: &InstallCommand, dry_run: bool, executor: F) -> Result<()>
where
    F: FnOnce(&InstallCommand) -> Result<()>,
{
    if dry_run {
        println!("[dry-run] {}", command.render());
        return Ok(());
    }

    println!("Executing: {}", command.render());
    executor(command)
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;
    use crate::config::InstallSpec;

    struct DummyPm;

    impl PackageManager for DummyPm {
        fn name(&self) -> &'static str {
            "winget"
        }

        fn executable(&self) -> &'static str {
            "winget"
        }

        fn config_keys(&self) -> &'static [&'static str] {
            &["winget"]
        }

        fn build_install_cmd(
            &self,
            identifier: &str,
            _version: Option<&str>,
        ) -> Result<InstallCommand> {
            Ok(InstallCommand::new(
                "winget",
                vec!["install".to_string(), identifier.to_string()],
            ))
        }
    }

    #[test]
    fn parse_target_with_inline_version() {
        let parsed = parse_target_spec("python:3.12").expect("must parse");
        assert_eq!(parsed.0, "python");
        assert_eq!(parsed.1.as_deref(), Some("3.12"));
    }

    #[test]
    fn parse_target_rejects_empty_version() {
        let err = parse_target_spec("python:").expect_err("must fail");
        assert!(err.to_string().contains("Version after `:` is empty"));
    }

    #[test]
    fn resolve_identifier_prefers_specific_manager_mapping() {
        let spec = InstallSpec::Detailed {
            version: Some("3.12".to_string()),
            identifiers: [
                ("winget".to_string(), "Python.Python.3.12".to_string()),
                ("default".to_string(), "python".to_string()),
            ]
            .into_iter()
            .collect(),
        };

        let resolved = resolve_identifier(&spec, &DummyPm, "python");
        assert_eq!(resolved, "Python.Python.3.12");
    }

    #[test]
    fn resolve_identifier_uses_global_string_without_changes() {
        let spec = InstallSpec::Identifier("My.Mixed.Case.Identifier".to_string());
        let resolved = resolve_identifier(&spec, &DummyPm, "python");
        assert_eq!(resolved, "My.Mixed.Case.Identifier");
    }

    #[test]
    fn dry_run_does_not_execute_installer() {
        let command = InstallCommand::new(
            "fake-installer",
            vec!["install".to_string(), "pkg".to_string()],
        );
        let result = execute_or_print_dry_run(&command, true, |_| {
            panic!("executor must not be called in dry-run mode");
        });
        assert!(result.is_ok());
    }

    #[test]
    fn non_dry_run_executes_installer() {
        let command = InstallCommand::new(
            "fake-installer",
            vec!["install".to_string(), "pkg".to_string()],
        );
        let result = execute_or_print_dry_run(&command, false, |_| Ok(()));
        assert!(result.is_ok());
    }
}
