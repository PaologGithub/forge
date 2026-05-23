use rhai::{CustomType, TypeBuilder};


/// Main helper for the ForgeRhai language. 
///  Adds the "project" struct with many functionnalities
#[derive(Clone, CustomType)]
#[rhai_type(name = "Project")]
pub struct ForgeRhaiProject {
    #[rhai_type(readonly)]
    pub version: &'static str
}

impl ForgeRhaiProject {
    pub fn new() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION")
        }
    }
}