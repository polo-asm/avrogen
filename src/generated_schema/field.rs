use crate::DateLibrary;
use crate::Result;
use apache_avro::schema::*;
use std::fmt::Write;

use super::{field_default_value::FieldDefault, field_type::*, global::*, schema::GeneratedType, ProcessSettings};

#[derive(Debug)]
pub struct GeneratedStructFields {
    parent_struct_fullname: String,

    name: SanitizedName,

    doc: String,

    type_name: String,

    serde_with_line: Option<&'static str>,

    default: Option<FieldDefault>,
}

impl GeneratedStructFields {
    pub fn has_default(&self) -> bool {
        self.default.is_some()
    }

    pub fn from(
        field: &RecordField,
        structure_name: &SanitizedName,
        settings: &ProcessSettings,
        extra_types: &mut Vec<GeneratedType>,
    ) -> Result<Self> {
        let field_name = SanitizedName::from_field(&field.name);
        let naming = StructFieldName {
            struct_name: &structure_name.sanitized_name,
            field_name: &field.name,
        };
        let field_type = get_field_type(&field.schema, settings, &naming, extra_types)?;

        let doc = format_doc(&field.doc, "    ")?;
        let serde_with_line = get_serde_with(field, settings);

        let default = match &field.default {
            None => None,
            Some(val) => Some(FieldDefault::from(val, &field.schema,settings)?),
        };

        Ok(GeneratedStructFields {
            name: field_name,
            parent_struct_fullname: structure_name.sanitized_name.to_owned(),
            type_name: field_type,
            serde_with_line,
            doc,
            default,
        })
    }

    pub fn write_struct_declaration_content(&self) -> Result<String> {
        let mut content = self.doc.to_owned();

        if let Some(line) = &self.serde_with_line {
            writeln!(content, "    {line}")?;
        };
        if self.name.is_sanitized {
            writeln!(
                content,
                "    #[serde(rename = \"{}\")]",
                self.name.original_name
            )?
        }
        if self.default.is_some() {
            writeln!(
                content,
                "    #[serde(default = \"{}::default_{}\")]",
                self.parent_struct_fullname, self.name.sanitized_name
            )?;
        }
        writeln!(
            content,
            "    pub {}: {},",
            self.name.sanitized_name, self.type_name
        )?;

        Ok(content)
    }

    pub fn write_struct_default_method_content(&self) -> Result<Option<String>> {
        match &self.default {
            Some(default) => {
                let default_value_str = default
                    .write_content()
                    .map_err(|e| format!("{} => {e}", self.name.sanitized_name))?;

                Ok(format!(
                    "\n    #[inline(always)]\n    pub fn default_{}() -> {} {{\n        {}\n    }}\n",
                    self.name.sanitized_name, self.type_name, default_value_str
                )
                .into())
            }
            None => Ok(None),
        }
    }
}

fn get_serde_with(field: &RecordField, settings: &ProcessSettings) -> Option<&'static str> {
    match (settings.date_library, &field.schema) {
        (DateLibrary::Chrono, Schema::TimeMillis | Schema::TimestampMillis | Schema::LocalTimestampMillis) => {
            Some("#[serde(with = \"chrono::naive::serde::ts_milliseconds\")]")
        }
        (DateLibrary::Chrono, Schema::TimeMicros | Schema::TimestampMicros | Schema::LocalTimestampMicros) => {
            Some("#[serde(with = \"chrono::naive::serde::ts_microseconds\")]")
        }

        (DateLibrary::Jiff, Schema::TimeMillis | Schema::TimestampMillis | Schema::LocalTimestampMillis) => {
            Some("#[serde(with = \"jiff::fmt::serde::timestamp::millisecond::required\")]")
        }
        (DateLibrary::Jiff, Schema::TimeMicros | Schema::TimestampMicros | Schema::LocalTimestampMicros) => {
            Some("#[serde(with = \"jiff::fmt::serde::timestamp::microsecond::required\")]")
        }
        (DateLibrary::Jiff, Schema::Date) => {
            Some("#[serde(with = \"jiff::fmt::serde::timestamp::second::required\")]")
        }

        _ => None,
    }
}
