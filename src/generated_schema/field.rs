use crate::Result;
use apache_avro::schema::*;
use std::fmt::Write;

use super::{field_default_value::FieldDefault, field_type::*, global::*};

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
    pub fn from(
        field: &RecordField,
        structure_name: &SanitizedName,
        default_namespace: &Option<String>,
    ) -> Result<Self> {
        let field_name = SanitizedName::from_field(&field.name);
        let field_type = get_field_type(&field.schema, default_namespace)?;

        let doc = format_doc(&field.doc, "    ")?;
        let serde_with_line = get_serde_with(field)?;

        let default = match &field.default {
            None => None,
            Some(val) => Some(FieldDefault::from(&val, &field.schema)?),
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
        if let Some(_) = self.default {
            writeln!(
                content,
                "    #[serde(default=\"{}::default_{}\")]",
                self.parent_struct_fullname, self.name.sanitized_name
            )?;
        }
        writeln!(
            content,
            "    pub {}: {},",
            self.name.sanitized_name, self.type_name
        )?;

        return Ok(content);
    }

    pub fn write_struct_default_method_content(&self) -> Result<Option<String>> {
        match &self.default {
            Some(default) => {
                let default_value_str = default
                    .write_content()
                    .map_err(|e| format!("{} => {e}", self.name.sanitized_name))?;

                Ok(format!(
                    "    #[inline(always)]\r\n    pub fn default_{}() -> {} {{ {} }}\r\n\r\n",
                    self.name.sanitized_name, self.type_name, default_value_str
                )
                .into())
            }
            None => Ok(None),
        }
    }
}

fn get_serde_with(field: &RecordField) -> Result<Option<&'static str>> {
    Ok(match field.schema {
        Schema::Date => None,
        Schema::TimeMillis => Some("#[serde(with = \"chrono::naive::serde::ts_milliseconds\")]"),
        Schema::TimeMicros => Some("#[serde(with = \"chrono::naive::serde::ts_microseconds\")]"),
        Schema::TimestampMillis => {
            Some("#[serde(with = \"chrono::naive::serde::ts_milliseconds\")]")
        }
        Schema::TimestampMicros => {
            Some("#[serde(with = \"chrono::naive::serde::ts_microseconds\")]")
        }
        Schema::LocalTimestampMillis => {
            Some("#[serde(with = \"chrono::naive::serde::ts_milliseconds\")]")
        }
        Schema::LocalTimestampMicros => {
            Some("#[serde(with = \"chrono::naive::serde::ts_microseconds\")]")
        }
        _ => None,
    })
}
