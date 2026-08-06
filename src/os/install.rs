use std::process::{Command, Stdio};

use anyhow::{Context, Result, bail};

use crate::config::{InstallSpec, load_project_config};
#[cfg(test)]
use crate::os::package_manager::package_manager_from_name;
use crate::os::package_manager::{InstallCommand, PackageManager, detect_package_manager};

#[derive(Debug, Clone)]
struct InstallPlan {
    manager_name: String,
    identifier: String,
    requested_version: Option<String>,
    inline_overrode_config: bool,
    command: InstallCommand,
}

/// Entry point from CLI.
pub fn install_target(raw_spec: &str, dry_run: bool, yes: bool) -> Result<()> {
    let selected_manager = detect_package_manager()?;
    let plan = build_install_plan(raw_spec, selected_manager.as_ref(), yes)?;

    if plan.inline_overrode_config {
        println!("Inline version override applied.");
    }

    println!("Selected package manager: {}", plan.manager_name);
    println!("Resolved identifier: {}", plan.identifier);
    if let Some(version) = plan.requested_version.as_deref() {
        println!("Resolved version: {version}");
    } else {
        println!("Resolved version: latest available from package manager");
    }

    execute_or_print_dry_run(&plan.command, dry_run, execute_install)
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

fn build_install_plan(
    raw_spec: &str,
    manager: &dyn PackageManager,
    yes: bool,
) -> Result<InstallPlan> {
    let (logical_target, inline_version) = parse_target_spec(raw_spec)?;

    let mut configured_version: Option<String> = None;
    let mut identifier = logical_target.clone();

    if let Some(cfg) = load_project_config()? {
        if let Some((entry_name, spec)) = cfg.install_target_case_insensitive(&logical_target) {
            configured_version = spec.version().map(|version| version.to_string());
            identifier = resolve_identifier(spec, manager, &logical_target);
            println!(
                "Using install config `{}` from {}",
                entry_name,
                cfg.path.display()
            );
        }
    }

    let inline_overrode_config = inline_version.is_some() && configured_version.is_some();
    let requested_version = inline_version.or(configured_version.clone());

    let mut command = manager.build_install_cmd(&identifier, requested_version.as_deref())?;
    if yes {
        manager.apply_yes_flag(&mut command);
    }

    Ok(InstallPlan {
        manager_name: manager.name().to_string(),
        identifier,
        requested_version,
        inline_overrode_config,
        command,
    })
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
    use std::ffi::OsString;

    use proptest::prelude::*;
    use serial_test::serial;

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

    struct EnvGuard {
        key: &'static str,
        original: Option<OsString>,
    }

    impl EnvGuard {
        fn set(key: &'static str, value: &str) -> Self {
            let original = std::env::var_os(key);
            // SAFETY: tests using this helper are marked `serial`, so there is no
            // concurrent environment mutation within this process.
            unsafe { std::env::set_var(key, value) };
            Self { key, original }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            match &self.original {
                Some(value) => {
                    // SAFETY: see `EnvGuard::set`; restoration happens in the same serial test.
                    unsafe { std::env::set_var(self.key, value) };
                }
                None => {
                    // SAFETY: see `EnvGuard::set`; restoration happens in the same serial test.
                    unsafe { std::env::remove_var(self.key) };
                }
            }
        }
    }

    fn plan_from_env_override(raw_spec: &str, yes: bool) -> Result<InstallPlan> {
        let raw = std::env::var("QBIT_PACKAGE_MANAGER")
            .expect("QBIT_PACKAGE_MANAGER must be set for this test");
        let manager = package_manager_from_name(&raw)
            .ok_or_else(|| anyhow::anyhow!("unknown package manager in test: {raw}"))?;
        build_install_plan(raw_spec, manager.as_ref(), yes)
    }

    #[test]
    fn parse_target_with_inline_version() {
        let parsed = parse_target_spec("python:3.12").expect("must parse");
        assert_eq!(parsed.0, "python");
        assert_eq!(parsed.1.as_deref(), Some("3.12"));
    }

    #[test]
    fn parse_target_with_trimmed_values() {
        let parsed = parse_target_spec("  chrome:127.0.0.0   ").expect("must parse");
        assert_eq!(parsed.0, "chrome");
        assert_eq!(parsed.1.as_deref(), Some("127.0.0.0"));
    }

    #[test]
    fn parse_target_without_version_is_trimmed() {
        let parsed = parse_target_spec("  python  ").expect("must parse");
        assert_eq!(parsed.0, "python");
        assert_eq!(parsed.1, None);
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

    #[test]
    #[serial]
    fn build_plan_uses_winget_from_env_override() {
        let _guard = EnvGuard::set("QBIT_PACKAGE_MANAGER", "winget");
        let plan = plan_from_env_override("Python.Python.3.12:3.12", true).expect("plan");

        assert_eq!(plan.manager_name, "winget");
        assert_eq!(plan.command.program, "winget");
        assert_eq!(
            plan.command.args.first().map(String::as_str),
            Some("install")
        );
        assert!(
            plan.command
                .args
                .iter()
                .any(|arg| arg == "--accept-source-agreements")
        );
        assert!(
            plan.command
                .args
                .iter()
                .any(|arg| arg == "--accept-package-agreements")
        );
    }

    #[test]
    #[serial]
    fn build_plan_uses_apt_from_env_override_with_version() {
        let _guard = EnvGuard::set("QBIT_PACKAGE_MANAGER", "apt");
        let plan = plan_from_env_override("python:3.12", false).expect("plan");

        if plan.command.program == "sudo" {
            assert_eq!(
                plan.command.args.first().map(String::as_str),
                Some("apt-get")
            );
            assert_eq!(
                plan.command.args.get(1).map(String::as_str),
                Some("install")
            );
        } else {
            assert_eq!(plan.command.program, "apt-get");
            assert_eq!(
                plan.command.args.first().map(String::as_str),
                Some("install")
            );
        }

        assert!(plan.command.render().contains("python=3.12"));
    }

    #[test]
    #[ignore = "Documenting intended behavior: install identifiers must preserve exact casing."]
    fn identifiers_preserve_casing_in_plan() {
        let spec = InstallSpec::Detailed {
            version: None,
            identifiers: [("winget".to_string(), "Python.Python.3.12".to_string())]
                .into_iter()
                .collect(),
        };

        let resolved = resolve_identifier(&spec, &DummyPm, "python");
        assert_eq!(resolved, "Python.Python.3.12");
    }

    proptest! {
        #[test]
        fn parse_target_spec_property_never_panics(
            name in "[^:\\r\\n]{0,24}",
            version in "[^\\r\\n]{0,24}",
            use_version in any::<bool>(),
            left_ws in "[ \\t]{0,4}",
            right_ws in "[ \\t]{0,4}",
        ) {
            let spec = if use_version {
                format!("{left_ws}{name}:{version}{right_ws}")
            } else {
                format!("{left_ws}{name}{right_ws}")
            };

            let result = parse_target_spec(&spec);
            if let Ok((resolved_name, _)) = result {
                prop_assert_eq!(resolved_name.trim(), resolved_name.as_str());
                prop_assert!(!resolved_name.is_empty());
            }
        }
    }
}
