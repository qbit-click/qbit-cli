use std::env;

use anyhow::{Context, Result};

use crate::config::{load_project_config, InstallSpec};
use crate::utils::shell;

/// Entry point from CLI.
pub fn install_target(raw_spec: &str) -> Result<()> {
    let (target, inline_version) = parse_target_spec(raw_spec);
    let mut requested_version = inline_version.map(|v| v.to_string());
    let mut resolved_identifier: Option<String> = None;

    if let Some(cfg) = load_project_config()? {
        if let Some(entry) = cfg.install_target(&target) {
            if requested_version.is_none() {
                requested_version = Some(entry.version().to_string());
            }
            resolved_identifier = Some(resolve_identifier(entry, &target));
            println!(
                "Requested `{}` version {} (defined in {})",
                target,
                entry.version(),
                cfg.path.display()
            );
        }
    }

    let resolved_name = resolved_identifier.unwrap_or_else(|| target.clone());
    let plan = build_plan(&resolved_name, requested_version.as_deref());
    println!("Preparing installation plan for `{}`...", plan.target);
    if let Some(version) = plan.version.as_deref() {
        println!("Desired version: {version}");
    }

    match plan.strategy {
        InstallStrategy::PackageManager { manager, command } => {
            println!("Detected package manager: {manager:?}");
            println!("Executing: {command}");
            shell::run_shell(&command)?;
        }
        InstallStrategy::Instructions { note } => {
            println!("Manual install instructions: {note}");
        }
    }

    Ok(())
}

fn parse_target_spec(spec: &str) -> (String, Option<String>) {
    if let Some((name, version)) = spec.split_once(':') {
        (name.trim().to_string(), Some(version.trim().to_string()))
    } else {
        (spec.trim().to_string(), None)
    }
}

/// Strategy for installing a given target.
#[derive(Debug, Clone)]
pub enum InstallStrategy {
    PackageManager {
        manager: PackageManager,
        command: String,
    },
    Instructions {
        note: String,
    },
}

/// High-level plan describing how qbit would install something.
#[derive(Debug, Clone)]
pub struct InstallPlan {
    pub target: String,
    pub version: Option<String>,
    pub strategy: InstallStrategy,
}

fn build_plan(target: &str, version: Option<&str>) -> InstallPlan {
    let normalized = target.to_lowercase();
    let manager = detect_package_manager();

    // For known targets we can enrich the suggestion.
    let hint = match normalized.as_str() {
        "java" | "jdk" => Some("Install Temurin/OpenJDK 21 (LTS)."),
        "python" => Some("Install CPython 3.11+ including pip."),
        _ => None,
    };

    let strategy = match manager {
        Some(pm) => {
            let cmd = pm.build_install_command(&normalized, version);
            let command_with_hint = if let Some(h) = hint {
                format!("{cmd}  # {h}")
            } else {
                cmd
            };
            InstallStrategy::PackageManager {
                manager: pm,
                command: command_with_hint,
            }
        }
        None => {
            let mut note = format!(
                "Automatic install not configured for `{}` on this platform.",
                target
            );
            if let Some(h) = hint {
                note.push(' ');
                note.push_str(h);
            }
            InstallStrategy::Instructions { note }
        }
    };

    InstallPlan {
        target: target.to_string(),
        version: version.map(|v| v.to_string()),
        strategy,
    }
}

fn detect_package_manager() -> Option<PackageManager> {
    if let Ok(override_name) = env::var("QBIT_PACKAGE_MANAGER") {
        return match override_name.to_lowercase().as_str() {
            "apt" | "apt-get" => Some(PackageManager::Apt),
            "brew" | "homebrew" => Some(PackageManager::Brew),
            "winget" => Some(PackageManager::Winget),
            "choco" | "chocolatey" => Some(PackageManager::Chocolatey),
            "scoop" => Some(PackageManager::Scoop),
            _ => None,
        };
    }

    if cfg!(target_os = "macos") {
        Some(PackageManager::Brew)
    } else if cfg!(target_os = "windows") {
        Some(PackageManager::Winget)
    } else if cfg!(target_os = "linux") {
        Some(PackageManager::Apt)
    } else {
        None
    }
}

/// Known package managers that qbit can orchestrate in the future.
#[derive(Debug, Clone, Copy)]
pub enum PackageManager {
    Apt,
    Brew,
    Winget,
    Chocolatey,
    Scoop,
}

impl PackageManager {
    fn base_command(self) -> &'static str {
        match self {
            PackageManager::Apt => "sudo apt-get install",
            PackageManager::Brew => "brew install",
            PackageManager::Winget => "winget install",
            PackageManager::Chocolatey => "choco install",
            PackageManager::Scoop => "scoop install",
        }
    }

    fn build_install_command(self, package: &str, version: Option<&str>) -> String {
        match (self, version) {
            (PackageManager::Apt, Some(ver)) => {
                format!("{} {}={}", self.base_command(), package, ver)
            }
            (PackageManager::Brew, Some(ver)) => {
                format!("{} {}@{}", self.base_command(), package, ver)
            }
            (PackageManager::Winget, Some(ver)) => format!(
                "{} {} --exact --accept-source-agreements --accept-package-agreements --version {}",
                self.base_command(),
                package,
                ver
            ),
            (PackageManager::Winget, None) => format!(
                "{} {} --exact --accept-source-agreements --accept-package-agreements",
                self.base_command(),
                package
            ),
            (PackageManager::Chocolatey, Some(ver)) => {
                format!("{} {} --version {}", self.base_command(), package, ver)
            }
            (PackageManager::Scoop, Some(ver)) => {
                format!("{} {}@{}", self.base_command(), package, ver)
            }
            (_, None) => format!("{} {}", self.base_command(), package),
            _ => format!("{} {}", self.base_command(), package),
        }
    }
}

fn resolve_identifier(spec: &InstallSpec, logical_name: &str) -> String {
    if let Some(pm) = detect_package_manager() {
        let key = match pm {
            PackageManager::Apt => "apt",
            PackageManager::Brew => "brew",
            PackageManager::Winget => "winget",
            PackageManager::Chocolatey => "choco",
            PackageManager::Scoop => "scoop",
        };
        if let Some(id) = spec.identifier(key) {
            return id.to_string();
        }
        if let Some(default) = spec.identifier("default") {
            return default.to_string();
        }
    }
    logical_name.to_string()
}
