use std::{error::Error, fs::read_to_string, path::PathBuf};

use rhai::{AST, Engine, Scope};

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

    pub fn has_function(&self, name: String, params: usize) -> bool {
        let ast = self
            .ast
            .as_ref()
            .expect("AST was normally some but is none.");

        ast.iter_functions()
            .any(|fn_def| fn_def.name == name && fn_def.params.len() == params)
    }

    pub fn run_main_function(&self, name: String) -> Result<i64, Box<dyn Error>> {
        let ast = self
            .ast
            .as_ref()
            .expect("AST was normally some but is none.");

        let reuslt = self.engine.call_fn::<i64>(&mut Scope::new(), ast, name, ())?;
        Ok(reuslt)
    }
}