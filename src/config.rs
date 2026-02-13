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

    pub fn install_target_case_insensitive(&self, name: &str) -> Option<(&str, &InstallSpec)> {
        self.data
            .install
            .iter()
            .find(|(key, _)| key.eq_ignore_ascii_case(name))
            .map(|(key, spec)| (key.as_str(), spec))
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
    Identifier(String),
    Detailed {
        #[serde(default)]
        version: Option<String>,
        #[serde(default)]
        identifiers: HashMap<String, String>,
    },
}

impl InstallSpec {
    pub fn version(&self) -> Option<&str> {
        self.configured_version()
    }

    pub fn configured_version(&self) -> Option<&str> {
        match self {
            InstallSpec::Identifier(_) => None,
            InstallSpec::Detailed { version, .. } => version.as_deref(),
        }
    }

    pub fn global_identifier(&self) -> Option<&str> {
        match self {
            InstallSpec::Identifier(identifier) => Some(identifier.as_str()),
            InstallSpec::Detailed { .. } => None,
        }
    }

    pub fn identifier(&self, manager: &str) -> Option<&str> {
        match self {
            InstallSpec::Identifier(identifier) => Some(identifier.as_str()),
            InstallSpec::Detailed { identifiers, .. } => identifiers
                .iter()
                .find(|(key, _)| key.eq_ignore_ascii_case(manager))
                .map(|(_, value)| value.as_str()),
        }
    }
}

pub fn load_project_config() -> Result<Option<LoadedProjectConfig>> {
    let current_dir = std::env::current_dir().context("resolving current directory for config")?;
    load_project_config_from_dir(&current_dir)
}

pub fn load_project_config_from_dir(base_dir: &Path) -> Result<Option<LoadedProjectConfig>> {
    for (file, format) in CONFIG_CANDIDATES {
        let path = base_dir.join(file);
        if !path.exists() {
            continue;
        }
        let content = fs::read_to_string(&path)
            .with_context(|| format!("reading project config at {}", path.display()))?;
        let data = match format {
            ConfigFormat::Yaml => parse_yaml_str(&content)
                .with_context(|| format!("parsing YAML config at {}", path.display()))?,
            ConfigFormat::Toml => parse_toml_str(&content)
                .with_context(|| format!("parsing TOML config at {}", path.display()))?,
        };
        return Ok(Some(LoadedProjectConfig { path, data }));
    }
    Ok(None)
}

pub(crate) fn parse_yaml_str(content: &str) -> Result<ProjectConfig> {
    Ok(serde_yaml::from_str(content)?)
}

pub(crate) fn parse_toml_str(content: &str) -> Result<ProjectConfig> {
    Ok(toml::from_str(content)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn install_lookup_is_case_insensitive() {
        let cfg = LoadedProjectConfig {
            path: PathBuf::from("qbit.yml"),
            data: ProjectConfig {
                scripts: HashMap::new(),
                install: [(
                    "PyThOn".to_string(),
                    InstallSpec::Identifier("Python.Python.3.12".to_string()),
                )]
                .into_iter()
                .collect(),
            },
        };

        let (_, entry) = cfg
            .install_target_case_insensitive("python")
            .expect("config entry");
        assert_eq!(entry.global_identifier(), Some("Python.Python.3.12"));
    }

    #[test]
    fn detailed_identifier_lookup_keeps_value_casing() {
        let spec = InstallSpec::Detailed {
            version: Some("3.12".to_string()),
            identifiers: [("Winget".to_string(), "Python.Python.3.12".to_string())]
                .into_iter()
                .collect(),
        };

        assert_eq!(spec.identifier("winget"), Some("Python.Python.3.12"));
    }

    #[test]
    fn parses_yaml_scripts_and_install_shapes() {
        let tmp = tempdir().expect("temp dir");
        let yaml = r#"scripts:
  hello: "echo hi"
  build:
    - "cargo build"
    - "cargo test"

install:
  node: "OpenJS.NodeJS"
  python:
    version: "3.12"
    identifiers:
      winget: "Python.Python.3.12"
      brew: "python@3.12"
"#;
        fs::write(tmp.path().join("qbit.yml"), yaml).expect("write yaml");

        let loaded = load_project_config_from_dir(tmp.path())
            .expect("parse yaml")
            .expect("config present");

        match loaded
            .script("hello")
            .expect("hello script should exist")
            .clone()
        {
            CommandList::Single(cmd) => assert_eq!(cmd, "echo hi"),
            CommandList::Multiple(_) => panic!("expected single command"),
        }

        assert_eq!(
            loaded
                .script("build")
                .expect("build script should exist")
                .commands(),
            vec!["cargo build".to_string(), "cargo test".to_string()]
        );

        let (_, node_spec) = loaded
            .install_target_case_insensitive("node")
            .expect("node install spec");
        assert_eq!(node_spec.global_identifier(), Some("OpenJS.NodeJS"));

        let (_, python_spec) = loaded
            .install_target_case_insensitive("python")
            .expect("python install spec");
        assert_eq!(python_spec.version(), Some("3.12"));
        assert_eq!(python_spec.identifier("winget"), Some("Python.Python.3.12"));
        assert_eq!(python_spec.identifier("brew"), Some("python@3.12"));
    }

    #[test]
    fn parses_toml_scripts_and_install_shapes() {
        let tmp = tempdir().expect("temp dir");
        let toml = r#"[scripts]
hello = "echo hi"
build = ["cargo build", "cargo test"]

[install]
node = "OpenJS.NodeJS"

[install.python]
version = "3.12"

[install.python.identifiers]
winget = "Python.Python.3.12"
brew = "python@3.12"
"#;
        fs::write(tmp.path().join("qbit.toml"), toml).expect("write toml");

        let loaded = load_project_config_from_dir(tmp.path())
            .expect("parse toml")
            .expect("config present");

        match loaded
            .script("hello")
            .expect("hello script should exist")
            .clone()
        {
            CommandList::Single(cmd) => assert_eq!(cmd, "echo hi"),
            CommandList::Multiple(_) => panic!("expected single command"),
        }

        assert_eq!(
            loaded
                .script("build")
                .expect("build script should exist")
                .commands(),
            vec!["cargo build".to_string(), "cargo test".to_string()]
        );

        let (_, node_spec) = loaded
            .install_target_case_insensitive("node")
            .expect("node install spec");
        assert_eq!(node_spec.global_identifier(), Some("OpenJS.NodeJS"));

        let (_, python_spec) = loaded
            .install_target_case_insensitive("python")
            .expect("python install spec");
        assert_eq!(python_spec.version(), Some("3.12"));
        assert_eq!(python_spec.identifier("winget"), Some("Python.Python.3.12"));
        assert_eq!(python_spec.identifier("brew"), Some("python@3.12"));
    }

    #[test]
    fn parse_yaml_str_handles_single_and_multiple_scripts() {
        let yaml = r#"scripts:
  one: "echo hi"
  many:
    - "cargo build"
    - "cargo test"
"#;
        let parsed = parse_yaml_str(yaml).expect("yaml parse");
        assert_eq!(
            parsed.scripts.get("one").expect("one command").commands(),
            vec!["echo hi".to_string()]
        );
        assert_eq!(
            parsed.scripts.get("many").expect("many command").commands(),
            vec!["cargo build".to_string(), "cargo test".to_string()]
        );
    }

    #[test]
    fn parse_toml_str_handles_install_identifier_and_detailed_forms() {
        let toml = r#"[install]
java = "Oracle.JavaRuntimeEnvironment"

[install.python]
version = "3.12"

[install.python.identifiers]
winget = "Python.Python.3.12"
default = "python"
"#;
        let parsed = parse_toml_str(toml).expect("toml parse");

        let java = parsed.install.get("java").expect("java install");
        assert_eq!(
            java.global_identifier(),
            Some("Oracle.JavaRuntimeEnvironment")
        );

        let python = parsed.install.get("python").expect("python install");
        assert_eq!(python.version(), Some("3.12"));
        assert_eq!(python.identifier("winget"), Some("Python.Python.3.12"));
        assert_eq!(python.identifier("default"), Some("python"));
    }
}
