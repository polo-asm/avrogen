

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub struct Machine {
    /// Name of the machine, should not be null
    #[serde(rename = "MachineName")]
    pub machine_name: String,
    /// technical identifier of the machine, should not be null
    #[serde(rename = "MachineIdentifier")]
    pub machine_identifier: i32,
    /// Date of the last update of this machine
    #[serde(with = "chrono::naive::serde::ts_microseconds")]
    #[serde(rename = "UpdateDate")]
    pub update_date: chrono::NaiveDateTime,
    /// Content is null when the machine structure has not been validated
    #[serde(rename = "Content")]
    pub content: crate::com::my_site::machines::MachineContent,
}

impl Machine {
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub struct MachineContent {
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "Length")]
    pub length: Option<apache_avro::Decimal>,
    #[serde(rename = "Width")]
    pub width: Option<apache_avro::Decimal>,
    #[serde(rename = "Height")]
    pub height: Option<apache_avro::Decimal>,
    #[serde(rename = "Parameters")]
    pub parameters: Option<Vec<crate::com::my_site::machines::Parameter>>,
    #[serde(rename = "Subsets")]
    pub subsets: Option<Vec<crate::com::my_site::machines::Subset>>,
}

impl MachineContent {
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Default)]
pub enum OverrideMode {
    #[default]
    Locked,
    WithPermission,
    Free,
}


#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub struct Parameter {
    #[serde(rename = "Tag")]
    pub tag: Option<String>,
    #[serde(rename = "Name")]
    pub name: String,
    /// Parameter technical identfier
    #[serde(rename = "Identifier")]
    pub identifier: i32,
    #[serde(rename = "UnitOfMeasurement")]
    #[serde(default = "Parameter::default_unit_of_measurement")]
    pub unit_of_measurement: Option<String>,
    /// Contains the value of the parameter in double type
    #[serde(rename = "Value")]
    pub value: Option<f64>,
    #[serde(rename = "OverrideMode")]
    pub override_mode: crate::com::my_site::machines::OverrideMode,
}

impl Parameter {
    #[inline(always)]
    pub fn default_unit_of_measurement() -> Option<String> { None }

}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub struct Subset {
    #[serde(rename = "Name")]
    pub name: String,
    /// Technical identifier, unique in the structure of the machine.
    #[serde(rename = "Identifier")]
    pub identifier: Option<String>,
    #[serde(rename = "Parameters")]
    pub parameters: Option<Vec<crate::com::my_site::machines::Parameter>>,
    #[serde(rename = "Subsets")]
    pub subsets: Option<Vec<crate::com::my_site::machines::Subset>>,
}

impl Subset {
}

