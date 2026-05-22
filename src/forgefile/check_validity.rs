use std::{error::Error, path::PathBuf};

use crate::forgerhai::engine::ForgeRhaiEngine;

/// Check the validity of the Forgefile content,
/// checking if rhai's Lexer accepts it.
pub fn check_validity(engine: &mut ForgeRhaiEngine, path: &PathBuf) -> Result<(), Box<dyn Error>> {
    engine.compile_file(path)?;

    Ok(())
}