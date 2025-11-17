mod cli;
mod developers;
mod os;
mod tools;
mod utils;
mod config;

fn main() {
    if let Ok(root) = std::env::var("QBIT_PROJECT_ROOT") {
        if let Err(e) = std::env::set_current_dir(&root) {
            eprintln!("warning: failed to switch to sandbox at {root}: {e}");
        }
    }
    cli::run();
}
