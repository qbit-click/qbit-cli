use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::Deserialize;

const CONFIG_CANDIDATES: &[(&str, ConfigFormat)] = &[
    ("qbit.yml", ConfigFormat::Yaml),
    ("qbit.yaml", ConfigFormat::Yaml),
    ("qbit.toml", ConfigFormat::Toml),
];

#[derive(Debug, Clone)]
pub enum ConfigFormat {
    Yaml,
    Toml,
}

#[derive(Debug, Clone)]
pub struct LoadedProjectConfig {
    pub path: PathBuf,
    pub data: ProjectConfig,
}

impl LoadedProjectConfig {
    pub fn script(&self, name: &str) -> Option<&CommandList> {
        self.data.scripts.get(name)
    }

    pub fn install_target(&self, name: &str) -> Option<&InstallSpec> {
        self.data.install.get(name)
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ProjectConfig {
    #[serde(default)]
    pub scripts: HashMap<String, CommandList>,
    #[serde(default)]
    pub install: HashMap<String, InstallSpec>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum CommandList {
    Single(String),
    Multiple(Vec<String>),
}

impl CommandList {
    pub fn commands(&self) -> Vec<String> {
        match self {
            CommandList::Single(cmd) => vec![cmd.clone()],
            CommandList::Multiple(cmds) => cmds.clone(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum InstallSpec {
    Version(String),
    Detailed {
        version: String,
        #[serde(default)]
        identifiers: HashMap<String, String>,
    },
}

impl InstallSpec {
    pub fn version(&self) -> &str {
        match self {
            InstallSpec::Version(v) => v,
            InstallSpec::Detailed { version, .. } => version,
        }
    }

    pub fn identifier(&self, manager: &str) -> Option<&str> {
        match self {
            InstallSpec::Version(_) => None,
            InstallSpec::Detailed { identifiers, .. } => identifiers.get(manager).map(String::as_str),
        }
    }
}

pub fn load_project_config() -> Result<Option<LoadedProjectConfig>> {
    for (file, format) in CONFIG_CANDIDATES {
        let path = Path::new(file);
        if !path.exists() {
            continue;
        }
        let content = fs::read_to_string(path)
            .with_context(|| format!("reading project config at {}", path.display()))?;
        let data = match format {
            ConfigFormat::Yaml => serde_yaml::from_str(&content)
                .with_context(|| format!("parsing YAML config at {}", path.display()))?,
            ConfigFormat::Toml => toml::from_str(&content)
                .with_context(|| format!("parsing TOML config at {}", path.display()))?,
        };
        return Ok(Some(LoadedProjectConfig {
            path: path.to_path_buf(),
            data,
        }));
    }
    Ok(None)
}
