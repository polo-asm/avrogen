use crate::Result;
use apache_avro::{schema::*, Schema};
use std::fmt::Write;
use std::string::*;
use std::*;
use crate::generated_schema::ProcessSettings;
use super::field::GeneratedStructFields;
use super::global::*;

#[derive(Debug)]
pub enum GeneratedType {
    None,

    Enum(GeneratedEnum),

    Struct(GeneratedStruct),

    Union(GeneratedUnion),
}

impl GeneratedType {
    pub fn produce_content(&self) -> Result<String> {
        match self {
            GeneratedType::Enum(x) => x.produce_content(),
            GeneratedType::Struct(x) => x.produce_content(),
            GeneratedType::Union(x) => x.produce_content(),
            GeneratedType::None => Ok("".to_string()),
        }
    }
    pub fn schema_name(&self) -> String {
        match self {
            GeneratedType::Enum(x) => x.name.original_name.to_owned(),
            GeneratedType::Struct(x) => x.name.original_name.to_owned(),
            GeneratedType::Union(x) => x.name.original_name.to_owned(),
            GeneratedType::None => "".to_owned(),
        }
    }
}

#[derive(Debug)]
pub struct GeneratedStruct {
    name: SanitizedName,

    schema_doc: String,

    fields: Vec<GeneratedStructFields>,
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

#[derive(Debug)]
pub struct GeneratedEnum {
    name: SanitizedName,

    schema_doc: String,

    default_record: Option<String>,

    records: Vec<String>,
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

#[derive(Debug)]
pub struct GeneratedUnionVariant {
    pub variant_name: SanitizedName,

    pub type_name: String,
}

#[derive(Debug)]
pub struct GeneratedUnion {
    name: SanitizedName,

    variants: Vec<GeneratedUnionVariant>,
}

impl GeneratedUnion {
    pub fn new(struct_name: &str, field_name: &str, variants: Vec<GeneratedUnionVariant>) -> Self {
        GeneratedUnion {
            name: union_type_name(struct_name, field_name),
            variants,
        }
    }

    pub fn type_name(&self) -> &str {
        &self.name.sanitized_name
    }

    pub fn produce_content(&self) -> Result<String> {
        let mut content_string = String::new();
        writeln!(
            content_string,
            "#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]"
        )?;
        writeln!(content_string, "#[serde(untagged)]")?;
        writeln!(content_string, "pub enum {} {{", self.name.sanitized_name)?;

        for variant in self.variants.iter() {
            writeln!(
                content_string,
                "    {}({}),",
                variant.variant_name.sanitized_name, variant.type_name
            )?;
        }
        write!(content_string, "}}\n\n")?;

        Ok(content_string)
    }
}

/// Nommage des enums générées pour les unions Avro multiples: {Struct}{Field}
pub fn union_type_name(struct_name: &str, field_name: &str) -> SanitizedName {
    let field_pascal = heck::ToUpperCamelCase::to_upper_camel_case(field_name);
    let combined = format!("{struct_name}{field_pascal}");

    SanitizedName {
        sanitized_name: combined.clone(),
        original_name: combined,
        is_sanitized: false,
    }
}

impl GeneratedType {
    /// Retourne le type principal généré, ainsi que d'éventuels types additionnels
    /// générés au passage (ex: l'enum d'un champ dont le type est une union multiple).
    pub fn generate_schema_struct(
        schema: &Schema,
        settings: &ProcessSettings,
    ) -> Result<(GeneratedType, Vec<GeneratedType>)> {
        match schema {
            Schema::Record(i) => {
                let (generated_struct, extra_types) = Self::treat_record_schema(i, settings)?;
                Ok((GeneratedType::Struct(generated_struct), extra_types))
            }
            Schema::Array(_) => todo!(),
            Schema::Map(_) => todo!(),
            Schema::Union(_) => todo!(),
            Schema::Enum(enum_schema) => {
                Self::treat_enum_schema(enum_schema).map(|e| (GeneratedType::Enum(e), vec![]))
            }
            Schema::Fixed(_) => todo!(),
            Schema::Decimal(_) => todo!(),
            Schema::Duration => todo!(),
            Schema::Ref { .. } => todo!(),
            _ => Ok((GeneratedType::None, vec![])),
        }
    }

    pub fn treat_enum_schema(enum_schema: &EnumSchema) -> Result<GeneratedEnum> {
        let schema_name = SanitizedName::from_type(&enum_schema.name.name);

        let schema_doc = format_doc(&enum_schema.doc, "")?;

        let default_record = enum_schema.default.to_owned();

        let records = enum_schema.symbols.iter().map(|e| e.to_owned()).collect();

        Ok(GeneratedEnum {
            name: schema_name,
            schema_doc,
            default_record,
            records,
        })
    }

    pub fn treat_record_schema(
        record_schema: &RecordSchema,
        settings: &ProcessSettings,
    ) -> Result<(GeneratedStruct, Vec<GeneratedType>)> {
        let schema_name = SanitizedName::from_type(&record_schema.name.name);

        let schema_doc = format_doc(&record_schema.doc, "")?;

        let mut extra_types: Vec<GeneratedType> = Vec::new();

        let fields: Result<Vec<GeneratedStructFields>> = record_schema
            .fields
            .iter()
            .map(|f| GeneratedStructFields::from(f, &schema_name, settings, &mut extra_types))
            .collect();

        Ok((
            GeneratedStruct {
                name: schema_name,
                schema_doc,
                fields: fields?,
            },
            extra_types,
        ))
    }
}
