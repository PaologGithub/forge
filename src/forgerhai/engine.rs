use std::{error::Error, fs::read_to_string, path::PathBuf};

use rhai::{AST, Engine};

use crate::forgerhai::{project::ForgeRhaiProject, scope::ForgeRhaiScope};

/// Holder for the rhai `Engine`,
///  that changes feature from it
///  to represent Forgefile structure
pub struct ForgeRhaiEngine {
    engine: Engine,
    scope: ForgeRhaiScope,
    ast: Option<AST>,

    project: ForgeRhaiProject
}

impl ForgeRhaiEngine {
    pub fn new() -> Self {
        let engine: Engine = Engine::new();
        let scope: ForgeRhaiScope = ForgeRhaiScope::new();
        let project: ForgeRhaiProject = ForgeRhaiProject::new();

        Self {
            engine,
            scope,
            ast: None,
            project
        }
    }

    pub fn compile_file(&mut self, path: &PathBuf) -> Result<(), Box<dyn Error>> {
        let content = read_to_string(path)?;
        
        let ast = self.engine.compile(content)?;
        self.ast = Some(ast);

        self.engine.build_type::<ForgeRhaiProject>();

        self.scope.init(self.project.clone());

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

    pub fn run_main_function(&mut self, name: String) -> Result<i64, Box<dyn Error>> {
        let ast = self
            .ast
            .as_ref()
            .expect("AST was normally some but is none.");

        let result = self.engine.call_fn::<i64>(self.scope.get_scope(), ast, name, ())?;
        Ok(result)
    }
}