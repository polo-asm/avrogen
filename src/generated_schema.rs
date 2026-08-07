
mod global;
pub mod namespace;
mod schema;
mod field_type;
mod field_default_value;
mod field;

pub struct ProcessSettings{
    pub default_namespace: Option<String>,
}

impl ProcessSettings {
    pub fn new(default_namespace: Option<String>) -> Self {
        Self {
            default_namespace,
        }
    }
}