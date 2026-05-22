use std::path::{Path, PathBuf};


/// Checks if `Forgefile` exists on the current
/// working directory.
/// 
/// Returns Some(Pathbuf) if it exists, and
/// None is if doesn't
pub fn check_availability() -> Option<PathBuf> {
    let path: &Path = Path::new("Forgefile");

    if path.exists() && path.is_file() {
        Some(path.to_path_buf())
    } else {
        None
    }
}