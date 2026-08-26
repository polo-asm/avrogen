mod struct_;
mod enum_;
mod union;

pub use enum_::GeneratedEnum;
pub use struct_::GeneratedStruct;
pub use union::{union_type_name, GeneratedUnion, GeneratedUnionVariant};

use crate::Result;
use apache_avro::{schema::*, Schema};
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


impl GeneratedType {
    /// Returns the main generated type, plus any additional type generated along the way
    /// (e.g. an enum for a field whose type is a multi-variant union).
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
