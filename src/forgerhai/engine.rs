use std::{error::Error, fs::read_to_string, path::PathBuf};

use rhai::{AST, Engine};

/// Holder for the rhai `Engine`,
///  that changes feature from it
///  to represent Forgefile structure
pub struct ForgeRhaiEngine {
    engine: Engine,

    ast: Option<AST>
}

impl ForgeRhaiEngine {
    pub fn new() -> Self {
        let engine: Engine = Engine::new();

        Self {
            engine,
            ast: None
        }
    }

    pub fn compile_file(&mut self, path: &PathBuf) -> Result<(), Box<dyn Error>> {
        let content = read_to_string(path)?;

        let ast = self.engine.compile(content)?;

        self.ast = Some(ast);

        Ok(()) 
    }
}