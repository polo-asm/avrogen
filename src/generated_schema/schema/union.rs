use crate::Result;
use crate::generated_schema::global::SanitizedName;
use std::fmt::Write;

#[derive(Debug)]
pub enum GeneratedUnionVariant {
    /// Unit variant (no data), used to represent `null`.
    Unit(SanitizedName),
    /// Variant carrying a value of the given Rust type.
    Tuple(SanitizedName, String),
}

#[derive(Debug)]
pub struct GeneratedUnion {
    pub(super) name: SanitizedName,

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
            match variant {
                GeneratedUnionVariant::Unit(variant_name) => writeln!(
                    content_string,
                    "    {},",
                    variant_name.sanitized_name
                )?,
                GeneratedUnionVariant::Tuple(variant_name, type_name) => writeln!(
                    content_string,
                    "    {}({}),",
                    variant_name.sanitized_name, type_name
                )?,
            }
        }
        write!(content_string, "}}\n\n")?;


        if let Some(first_variant) = self.variants.first() {
            writeln!(content_string, "impl Default for {} {{", self.name.sanitized_name)?;
            writeln!(content_string, "    fn default() -> Self {{")?;
            match first_variant {
                GeneratedUnionVariant::Unit(variant_name) => writeln!(
                    content_string,
                    "        {}::{}",
                    self.name.sanitized_name, variant_name.sanitized_name
                )?,
                GeneratedUnionVariant::Tuple(variant_name, _) => writeln!(
                    content_string,
                    "        {}::{}(Default::default())",
                    self.name.sanitized_name, variant_name.sanitized_name
                )?,
            }
            writeln!(content_string, "    }}")?;
            write!(content_string, "}}\n\n")?;
        }

        Ok(content_string)
    }
}

pub fn union_type_name(struct_name: &str, field_name: &str) -> SanitizedName {
    let field_pascal = heck::ToUpperCamelCase::to_upper_camel_case(field_name);
    let combined = format!("{struct_name}{field_pascal}");

    SanitizedName {
        sanitized_name: combined.clone(),
        original_name: combined,
        is_sanitized: false,
    }
}

