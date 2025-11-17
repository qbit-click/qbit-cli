use std::env;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

use anyhow::{bail, Context, Result};

/// Initialize a minimal JS/TS project by scaffolding package.json and src/index.js
pub fn init() -> Result<()> {
    ensure_project_config_file()?;
    ensure_package_json()?;
    ensure_src_tree()?;
    println!("JavaScript project scaffolded. Run `npm install` to add dependencies.");
    Ok(())
}

pub fn add_package(package: &str) -> Result<()> {
    ensure_package_json()?;
    run_package_manager(&["install", package])?;
    println!("Package `{package}` added via npm.");
    Ok(())
}

pub fn remove_package(package: &str) -> Result<()> {
    ensure_package_json()?;
    run_package_manager(&["uninstall", package])?;
    println!("Package `{package}` removed via npm.");
    Ok(())
}

pub fn run_script(script: &str) -> Result<()> {
    ensure_package_json()?;
    run_package_manager(&["run", script])?;
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
        .and_then(|path| path.file_name().map(|name| name.to_string_lossy().to_string()))
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| "qbit-app".to_string())
}

fn run_package_manager(args: &[&str]) -> Result<()> {
    let pm = resolve_package_manager()?;
    println!("{pm} {}", args.join(" "));
    let status = Command::new(&pm)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .with_context(|| format!("spawning {pm}"))?;

    if !status.success() {
        bail!(
            "{} command failed (code: {})",
            pm,
            status.code().unwrap_or_default()
        );
    }
    Ok(())
}

fn resolve_package_manager() -> Result<String> {
    if let Ok(explicit) = env::var("QBIT_JS_PM") {
        return Ok(explicit);
    }

    let candidates = ["npm", "pnpm", "yarn", "bun"];
    for cand in candidates {
        if command_available(cand) {
            return Ok(cand.to_string());
        }
    }

    bail!("No JS package manager found. Install npm/pnpm/yarn or set QBIT_JS_PM.");
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
