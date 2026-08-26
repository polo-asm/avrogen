use crate::Result;
use crate::generated_schema::global::SanitizedName;
use std::fmt::Write;

#[derive(Debug)]
pub struct GeneratedEnum {
    pub(super) name: SanitizedName,

    pub(super) schema_doc: String,

    pub(super) default_record: Option<String>,

    pub(super) records: Vec<String>,
}

impl GeneratedEnum {
    pub fn produce_content(&self) -> Result<String> {
        let mut content_string = self.schema_doc.to_owned();
        writeln!(
            content_string,
            "#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Default)]"
        )?;

        if self.name.is_sanitized {
            writeln!(
                content_string,
                "#[serde(rename = \"{}\")]",
                self.name.original_name
            )?
        }
        writeln!(content_string, "pub enum {} {{", self.name.sanitized_name)?;

        if  self.default_record.is_none() {
            writeln!(content_string, "    #[default]")?;
        }

        for enum_record in self.records.iter() {
            if let Some(default_value) = &self.default_record {
                if default_value == enum_record {
                    writeln!(content_string, "    #[default]")?;
                }
            }

            let record_name = SanitizedName::from_type(enum_record);

            if record_name.is_sanitized {
                writeln!(
                    content_string,
                    "    #[serde(rename = \"{}\")]",
                    record_name.original_name
                )?
            }
            writeln!(content_string, "    {},", record_name.sanitized_name)?;
        }
        write!(content_string, "}}\n\n")?;

        Ok(content_string)
    }
}

