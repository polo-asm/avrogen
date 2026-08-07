use crate::DateLibrary;

mod global;
pub mod namespace;
mod schema;
mod field_type;
mod field_default_value;
mod field;

pub struct ProcessSettings{
    pub default_namespace: Option<String>,
    pub date_library: DateLibrary,
}

impl ProcessSettings {
    pub fn new(default_namespace: Option<String>, date_library: DateLibrary) -> Self {
        Self {
            default_namespace,
            date_library,
        }
    }
}