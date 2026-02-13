use crate::developers::{dart, js, py};
use crate::os::install;
use crate::tools::runner;
use clap::{Parser, Subcommand};

/// Root CLI for qbit
#[derive(Parser)]
#[command(name = "qbit")]
#[command(about = "Multi-language package/project manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Install a system dependency (java, python, ...)
    Install {
        /// Package to install
        target: String,
        /// Print the resolved installer command without executing it
        #[arg(long)]
        dry_run: bool,
        /// Prefer non-interactive mode (adds `-y`/equivalent where supported)
        #[arg(long)]
        yes: bool,
    },
    /// Python-related commands
    Py {
        #[command(subcommand)]
        sub: PyCommands,
    },
    /// Run named scripts from qbit.yml/qbit.toml
    Run {
        /// Script name defined under `scripts`
        name: String,
    },
    /// JavaScript-related commands
    Js {
        #[command(subcommand)]
        sub: JsCommands,
    },
    /// Dart-related commands
    Dart {
        #[command(subcommand)]
        sub: DartCommands,
    },
}

/// Python subcommands
#[derive(Subcommand)]
pub enum PyCommands {
    /// Initialize a Python project (venv + requirements.txt)
    Init,
    /// Add a package
    Add {
        /// Package name
        package: String,
    },
    /// Remove a package
    Remove {
        /// Package name
        package: String,
    },
}

/// JavaScript subcommands
#[derive(Subcommand)]
pub enum JsCommands {
    /// Initialize a JS/TS project
    Init,
    /// Add a package via npm/yarn/pnpm (future)
    Add {
        /// Package name
        package: String,
    },
    /// Remove a package
    Remove {
        /// Package name
        package: String,
    },
    /// Run an npm/pnpm/yarn/bun script
    Run {
        /// Script name under package.json scripts
        script: String,
        /// Extra arguments forwarded to the script after `--`
        #[arg(last = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
}

/// Dart subcommands
#[derive(Subcommand)]
pub enum DartCommands {
    /// Initialize a Dart/Flutter project
    Init,
    /// Add one or more packages
    Add {
        /// Package names
        #[arg(required = true, num_args = 1..)]
        packages: Vec<String>,
    },
    /// Remove one or more packages
    Remove {
        /// Package names
        #[arg(required = true, num_args = 1..)]
        packages: Vec<String>,
    },
}

/// Dispatch after parse
pub fn run() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Install {
            target,
            dry_run,
            yes,
        } => {
            if let Err(e) = install::install_target(&target, dry_run, yes) {
                eprintln!("error (install): {e}");
                std::process::exit(1);
            }
        }
        Commands::Run { name } => {
            if let Err(e) = runner::run_named_script(&name) {
                eprintln!("error (run): {e}");
                std::process::exit(1);
            }
        }
        Commands::Py { sub } => match sub {
            PyCommands::Init => {
                if let Err(e) = py::init() {
                    eprintln!("error (init): {e}");
                    std::process::exit(1);
                }
            }
            PyCommands::Add { package } => {
                if let Err(e) = py::add_package(&package) {
                    eprintln!("error (add): {e}");
                    std::process::exit(1);
                }
            }
            PyCommands::Remove { package } => {
                if let Err(e) = py::remove_package(&package) {
                    eprintln!("error (remove): {e}");
                    std::process::exit(1);
                }
            }
        },
        Commands::Js { sub } => match sub {
            JsCommands::Init => {
                if let Err(e) = js::init() {
                    eprintln!("error (js init): {e}");
                    std::process::exit(1);
                }
            }
            JsCommands::Add { package } => {
                if let Err(e) = js::add_package(&package) {
                    eprintln!("error (js add): {e}");
                    std::process::exit(1);
                }
            }
            JsCommands::Remove { package } => {
                if let Err(e) = js::remove_package(&package) {
                    eprintln!("error (js remove): {e}");
                    std::process::exit(1);
                }
            }
            JsCommands::Run { script, args } => {
                if let Err(e) = js::run_script(&script, &args) {
                    eprintln!("error (js run): {e}");
                    std::process::exit(1);
                }
            }
        },
        Commands::Dart { sub } => match sub {
            DartCommands::Init => {
                if let Err(e) = dart::init() {
                    eprintln!("error (dart init): {e}");
                    std::process::exit(1);
                }
            }
            DartCommands::Add { packages } => {
                if let Err(e) = dart::add_packages(&packages) {
                    eprintln!("error (dart add): {e}");
                    std::process::exit(1);
                }
            }
            DartCommands::Remove { packages } => {
                if let Err(e) = dart::remove_packages(&packages) {
                    eprintln!("error (dart remove): {e}");
                    std::process::exit(1);
                }
            }
        },
    }
}
