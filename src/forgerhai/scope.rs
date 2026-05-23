use rhai::Scope;

/// Holder for the rhai `Scope`,
///  that adds new values like "project"
///  to add ForgeRhai functions and classes
pub struct ForgeRhaiScope {
    scope: Scope<'static>
}

impl ForgeRhaiScope {
    pub fn new() -> Self {
        let scope = Scope::new();

        Self {
            scope
        }
    }

    pub fn init(&mut self) {
        self.scope.set_value("project", "asd");
    }

    pub fn get_scope(&mut self) -> &mut Scope<'static> {
        &mut self.scope
    }
}