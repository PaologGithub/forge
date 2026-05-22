use rhai::Engine;

/// Holder for the rhai `Engine`,
///  that changes feature from it
///  to represent Forgefile structure
pub struct ForgeRhaiEngine {
    engine: Engine
}

impl ForgeRhaiEngine {
    pub fn new() -> Self {
        let engine: Engine = Engine::new();

        Self {
            engine
        }
    }

    pub fn run_file(&self) {
        self.engine.eval_file::<i64>("test.rhai".into()).unwrap();
    }
}