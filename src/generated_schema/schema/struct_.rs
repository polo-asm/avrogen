use crate::Result;
use crate::generated_schema::field::GeneratedStructFields;
use crate::generated_schema::global::SanitizedName;
use std::fmt::Write;

#[derive(Debug)]
pub struct GeneratedStruct {
    pub(super) name: SanitizedName,

    pub(super) schema_doc: String,

    pub(super) fields: Vec<GeneratedStructFields>,
}

impl GeneratedStruct {
    pub fn produce_content(&self) -> Result<String> {
        let mut content_string = self.schema_doc.to_owned();
        writeln!(
            content_string,
            "#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Default)]"
        )?;
        writeln!(content_string, "#[serde(default)]")?;
        if self.name.is_sanitized {
            writeln!(
                content_string,
                "#[serde(rename = \"{}\")]",
                self.name.original_name
            )?
        }
        writeln!(content_string, "pub struct {} {{", self.name.sanitized_name)?;

        for field in self.fields.iter() {
            let field_declaration = field.write_struct_declaration_content()?;
            content_string.push_str(&field_declaration);
        }
        write!(content_string, "}}\n\n")?;

        write!(content_string, "impl {} {{", self.name.sanitized_name)?;

        if self.fields.iter().any(|f| f.has_default()) {
            for field in self.fields.iter() {
                let field_default_method = field.write_struct_default_method_content()?;
                if let Some(field_default_method) = field_default_method {
                    content_string.push_str(&field_default_method);
                }
            }
        }
        write!(content_string, "}}\n\n")?;

        Ok(content_string)
    }
}

