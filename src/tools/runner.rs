use anyhow::{bail, Result};

use crate::config::load_project_config;
use crate::utils::shell;

pub fn run_named_script(name: &str) -> Result<()> {
    let Some(cfg) = load_project_config()? else {
        bail!("No qbit.yml/qbit.toml file found in the current directory.");
    };

    let Some(entry) = cfg.script(name) else {
        bail!(
            "Script `{}` not found in {}",
            name,
            cfg.path.display()
        );
    };

    let commands = entry.commands();
    shell::run_commands(&format!("script:{name}"), &commands)?;
    Ok(())
}
