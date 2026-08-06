use std::path::PathBuf;

/// Shared metadata about the project root to help language-specific managers.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ProjectContext {
    pub root: PathBuf,
}

impl ProjectContext {
    #[allow(dead_code)]
    pub fn from_current_dir() -> Self {
        let root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        Self { root }
    }
}
