use std::env;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

use anyhow::{Context, Result, bail};

/// Initialize a minimal JS/TS project by scaffolding package.json and src/index.js
pub fn init() -> Result<()> {
    ensure_project_config_file()?;
    ensure_package_json()?;
    ensure_src_tree()?;
    println!(
        "JavaScript project scaffolded. Run your package manager install command to add dependencies."
    );
    Ok(())
}

pub fn add_package(package: &str) -> Result<()> {
    ensure_package_json()?;
    let pm = resolve_package_manager()?;
    let command = build_add_command(pm, package)?;
    run_package_manager(&command)?;
    println!("Package `{package}` added via {}.", pm.name());
    Ok(())
}

pub fn remove_package(package: &str) -> Result<()> {
    ensure_package_json()?;
    let pm = resolve_package_manager()?;
    let command = build_remove_command(pm, package)?;
    run_package_manager(&command)?;
    println!("Package `{package}` removed via {}.", pm.name());
    Ok(())
}

pub fn run_script(script: &str, script_args: &[String]) -> Result<()> {
    ensure_package_json()?;
    let pm = resolve_package_manager()?;
    let command = build_run_command(pm, script, script_args)?;
    run_package_manager(&command)?;
    Ok(())
}

fn ensure_package_json() -> Result<()> {
    if Path::new("package.json").exists() {
        println!("package.json already exists");
        return Ok(());
    }

    let name = project_name();
    let package = format!(
        r#"{{
  "name": "{name}",
  "version": "0.1.0",
  "type": "module",
  "scripts": {{
    "start": "node src/index.js",
    "build": "echo \"Define build tooling\" && exit 0"
  }},
  "dependencies": {{}}
}}
"#
    );
    fs::write("package.json", package.as_bytes()).context("writing package.json")?;
    println!("Created package.json");
    Ok(())
}

fn ensure_src_tree() -> Result<()> {
    let src = Path::new("src");
    if !src.exists() {
        fs::create_dir_all(src).context("creating src directory")?;
        println!("Created src/ directory");
    }

    let entry = src.join("index.js");
    if !entry.exists() {
        let content = r#"console.log("Hello from qbit js init!");"#;
        fs::write(&entry, content.as_bytes()).context("writing src/index.js")?;
        println!("Created src/index.js");
    } else {
        println!("src/index.js already exists");
    }

    Ok(())
}

fn ensure_project_config_file() -> Result<()> {
    if Path::new("qbit.yml").exists()
        || Path::new("qbit.yaml").exists()
        || Path::new("qbit.toml").exists()
    {
        return Ok(());
    }

    let template = r#"scripts:
  dev: "npm run dev"
  lint:
    - "qbit py init"
    - "python -m flake8"

install:
  postgres:
    version: "15"
    identifiers:
      apt: "postgresql"
      winget: "PostgreSQL.PostgreSQL"
  redis:
    version: "7.2"
    identifiers:
      apt: "redis-server"
      winget: "Redis.Redis-CLI"
"#;
    fs::write("qbit.yml", template.as_bytes()).context("writing qbit.yml template")?;
    println!("Created qbit.yml");
    Ok(())
}

fn project_name() -> String {
    std::env::current_dir()
        .ok()
        .and_then(|path| {
            path.file_name()
                .map(|name| name.to_string_lossy().to_string())
        })
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| "qbit-app".to_string())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct JsCommandSpec {
    pm: JsPackageManager,
    args: Vec<String>,
}

impl JsCommandSpec {
    fn render(&self) -> String {
        format!("{} {}", self.pm.executable(), self.args.join(" "))
    }
}

fn run_package_manager(command: &JsCommandSpec) -> Result<()> {
    println!("Using JavaScript package manager: {}", command.pm.name());
    println!("{}", command.render());
    let status = Command::new(command.pm.executable())
        .args(&command.args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .with_context(|| format!("spawning {}", command.pm.executable()))?;

    if !status.success() {
        bail!(
            "{} command failed (code: {})",
            command.pm.name(),
            status.code().unwrap_or_default()
        );
    }
    Ok(())
}

fn resolve_package_manager() -> Result<JsPackageManager> {
    let env_override = env::var("QBIT_JS_PM").ok();
    let lockfile = detect_by_lockfile();
    resolve_package_manager_from_state(env_override.as_deref(), lockfile, |pm| {
        command_available(pm.executable())
    })
}

fn detect_by_lockfile() -> Option<(JsPackageManager, &'static str)> {
    lockfile_priority()
        .iter()
        .find(|(lockfile, _)| Path::new(lockfile).exists())
        .map(|(lockfile, pm)| (*pm, *lockfile))
}

fn lockfile_priority() -> [(&'static str, JsPackageManager); 4] {
    [
        ("bun.lockb", JsPackageManager::Bun),
        ("pnpm-lock.yaml", JsPackageManager::Pnpm),
        ("yarn.lock", JsPackageManager::Yarn),
        ("package-lock.json", JsPackageManager::Npm),
    ]
}

fn resolve_package_manager_from_state<F>(
    env_override: Option<&str>,
    detected_lockfile: Option<(JsPackageManager, &'static str)>,
    is_available: F,
) -> Result<JsPackageManager>
where
    F: Fn(JsPackageManager) -> bool,
{
    if let Some(raw) = env_override {
        let pm = JsPackageManager::parse(raw).ok_or_else(|| {
            anyhow::anyhow!(
                "Unsupported QBIT_JS_PM value `{}`. Supported values: bun, pnpm, yarn, npm.",
                raw
            )
        })?;
        if !is_available(pm) {
            bail!(
                "QBIT_JS_PM is set to `{}`, but `{}` is not available in PATH. Install it or unset QBIT_JS_PM.",
                pm.name(),
                pm.executable()
            );
        }
        return Ok(pm);
    }

    if let Some((pm, lockfile)) = detected_lockfile {
        if is_available(pm) {
            return Ok(pm);
        }
        bail!(
            "Detected `{}` lockfile (`{}`), but `{}` is not available in PATH. Install {} or set QBIT_JS_PM to another installed manager.",
            pm.name(),
            lockfile,
            pm.executable(),
            pm.name()
        );
    }

    for pm in [
        JsPackageManager::Bun,
        JsPackageManager::Pnpm,
        JsPackageManager::Yarn,
        JsPackageManager::Npm,
    ] {
        if is_available(pm) {
            return Ok(pm);
        }
    }

    bail!(
        "No JavaScript package manager found in PATH. Install one of: bun, pnpm, yarn, npm. You can also set QBIT_JS_PM=bun|pnpm|yarn|npm."
    );
}

fn command_available(cmd: &str) -> bool {
    Command::new(cmd)
        .arg("--version")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|st| st.success())
        .unwrap_or(false)
}

fn build_add_command(pm: JsPackageManager, package: &str) -> Result<JsCommandSpec> {
    let package = package.trim();
    if package.is_empty() {
        bail!("Package name must be non-empty.");
    }
    Ok(JsCommandSpec {
        pm,
        args: pm.add_args([package]),
    })
}

fn build_remove_command(pm: JsPackageManager, package: &str) -> Result<JsCommandSpec> {
    let package = package.trim();
    if package.is_empty() {
        bail!("Package name must be non-empty.");
    }
    Ok(JsCommandSpec {
        pm,
        args: pm.remove_args([package]),
    })
}

fn build_run_command(
    pm: JsPackageManager,
    script: &str,
    script_args: &[String],
) -> Result<JsCommandSpec> {
    let script = script.trim();
    if script.is_empty() {
        bail!("Script name must be non-empty.");
    }
    Ok(JsCommandSpec {
        pm,
        args: pm.run_args(script, script_args),
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum JsPackageManager {
    Bun,
    Pnpm,
    Yarn,
    Npm,
}

impl JsPackageManager {
    fn parse(raw: &str) -> Option<Self> {
        match raw.trim().to_ascii_lowercase().as_str() {
            "bun" => Some(Self::Bun),
            "pnpm" => Some(Self::Pnpm),
            "yarn" => Some(Self::Yarn),
            "npm" => Some(Self::Npm),
            _ => None,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::Bun => "bun",
            Self::Pnpm => "pnpm",
            Self::Yarn => "yarn",
            Self::Npm => "npm",
        }
    }

    fn executable(self) -> &'static str {
        self.name()
    }

    fn add_args<I>(self, packages: I) -> Vec<String>
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        let mut args = vec![self.add_verb().to_string()];
        args.extend(packages.into_iter().map(|p| p.as_ref().to_string()));
        args
    }

    fn remove_args<I>(self, packages: I) -> Vec<String>
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        let mut args = vec![self.remove_verb().to_string()];
        args.extend(packages.into_iter().map(|p| p.as_ref().to_string()));
        args
    }

    fn run_args(self, script: &str, script_args: &[String]) -> Vec<String> {
        let mut args = vec!["run".to_string(), script.to_string()];
        if !script_args.is_empty() {
            args.push("--".to_string());
            args.extend(script_args.iter().cloned());
        }
        args
    }

    fn add_verb(self) -> &'static str {
        match self {
            Self::Npm => "install",
            Self::Pnpm => "add",
            Self::Yarn => "add",
            Self::Bun => "add",
        }
    }

    fn remove_verb(self) -> &'static str {
        match self {
            Self::Npm => "uninstall",
            Self::Pnpm => "remove",
            Self::Yarn => "remove",
            Self::Bun => "remove",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn npm_and_yarn_use_expected_verbs() {
        assert_eq!(JsPackageManager::Npm.add_verb(), "install");
        assert_eq!(JsPackageManager::Npm.remove_verb(), "uninstall");
        assert_eq!(JsPackageManager::Yarn.add_verb(), "add");
        assert_eq!(JsPackageManager::Yarn.remove_verb(), "remove");
    }

    #[test]
    fn run_args_forward_extra_args_after_separator() {
        let args = JsPackageManager::Pnpm.run_args(
            "build",
            &[
                "--watch".to_string(),
                "--mode".to_string(),
                "dev".to_string(),
            ],
        );
        assert_eq!(
            args,
            vec![
                "run".to_string(),
                "build".to_string(),
                "--".to_string(),
                "--watch".to_string(),
                "--mode".to_string(),
                "dev".to_string()
            ]
        );
    }

    #[test]
    fn lockfile_priority_prefers_bun_over_others() {
        let resolved = lockfile_priority()
            .iter()
            .find(|(candidate, _)| {
                ["yarn.lock", "package-lock.json", "bun.lockb"]
                    .iter()
                    .any(|name| name == candidate)
            })
            .map(|(candidate, pm)| (*pm, *candidate));
        let (pm, lockfile) = resolved.expect("lockfile resolution");
        assert_eq!(pm.name(), "bun");
        assert_eq!(lockfile, "bun.lockb");
    }

    #[test]
    fn resolve_pm_prefers_env_override_when_available() {
        let pm = resolve_package_manager_from_state(
            Some("npm"),
            Some((JsPackageManager::Bun, "bun.lockb")),
            |candidate| candidate == JsPackageManager::Npm,
        )
        .expect("must resolve");
        assert_eq!(pm, JsPackageManager::Npm);
    }

    #[test]
    fn resolve_pm_uses_lockfile_when_no_override() {
        let pm = resolve_package_manager_from_state(
            None,
            Some((JsPackageManager::Yarn, "yarn.lock")),
            |candidate| candidate == JsPackageManager::Yarn,
        )
        .expect("must resolve");
        assert_eq!(pm, JsPackageManager::Yarn);
    }

    #[test]
    fn build_command_for_lockfile_selected_pm() {
        let pm = resolve_package_manager_from_state(
            None,
            Some((JsPackageManager::Pnpm, "pnpm-lock.yaml")),
            |_| true,
        )
        .expect("must resolve");
        let command = build_add_command(pm, "axios").expect("command");
        assert_eq!(command.pm, JsPackageManager::Pnpm);
        assert_eq!(command.args, vec!["add".to_string(), "axios".to_string()]);
    }
}
