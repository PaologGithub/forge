use rhai::Scope;

use crate::forgerhai::project::ForgeRhaiProject;

/// Holder for the rhai `Scope`,
///  that adds new values like "project"
///  to add ForgeRhai functions and classes
pub struct ForgeRhaiScope {
    scope: Scope<'static>,
}

impl ForgeRhaiScope {
    pub fn new() -> Self {
        let scope = Scope::new();

        Self {
            scope
        }
    }

    pub fn init(&mut self, project: ForgeRhaiProject) {
        self.scope.set_value("project", project);
    }

    pub fn get_scope(&mut self) -> &mut Scope<'static> {
        &mut self.scope
    }
}