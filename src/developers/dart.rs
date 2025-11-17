use anyhow::Result;

/// Placeholder Dart helpers for future `dart pub` automation.
pub fn init() -> Result<()> {
    println!("`qbit dart init` is not implemented yet. Planned: create pubspec.yaml + scaffold.");
    Ok(())
}

pub fn add_package(package: &str) -> Result<()> {
    println!(
        "`qbit dart add {package}` is not implemented yet. Planned: dart pub add + lock sync."
    );
    Ok(())
}

pub fn remove_package(package: &str) -> Result<()> {
    println!(
        "`qbit dart remove {package}` is not implemented yet. Planned: dart pub remove + lock sync."
    );
    Ok(())
}
