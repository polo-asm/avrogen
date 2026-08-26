use crate::Result;
use crate::DateLibrary;
use apache_avro::schema::*;
use crate::generated_schema::ProcessSettings;
use super::global::SanitizedName;
use super::schema::{GeneratedType, GeneratedUnion, GeneratedUnionVariant};

/// Identifie le champ (structure + nom du champ) pour lequel un type est généré.
/// Sert notamment à nommer les enums générées pour les unions multiples: `{Struct}{Field}`.
pub struct StructFieldName<'a> {
    pub struct_name: &'a str,
    pub field_name: &'a str,
}

pub fn is_nullable(schema: &Schema) -> bool {
    match schema {
        Schema::Null => true,
        Schema::Union(union_schema) => union_schema.variants().iter().any( is_nullable),
        _ => false,
    }
}

pub fn get_field_type(
    schema: &Schema,
    settings: &ProcessSettings,
    naming: &StructFieldName,
    extra_types: &mut Vec<GeneratedType>,
) -> Result<String> {
    let datetime_type = match settings.date_library {
        DateLibrary::Chrono => "chrono::NaiveDateTime",
        DateLibrary::Jiff => "jiff::Timestamp",
    };
    match schema {
        Schema::Null => Ok("null".to_string()),
        Schema::Boolean => Ok("bool".to_string()),
        Schema::Int => Ok("i32".to_string()),
        Schema::Long => Ok("i64".to_string()),
        Schema::Float => Ok("f32".to_string()),
        Schema::Double => Ok("f64".to_string()),
        Schema::Bytes => Ok("Vec<u8>".to_string()),
        Schema::String => Ok("String".to_string()),
        Schema::Array(array_schema) => get_field_type_array(&array_schema.items, settings, naming, extra_types),
        Schema::Map(map_schema) => get_field_type_map(&map_schema.types, settings, naming, extra_types),
        Schema::Union(union_schema) => get_field_type_union(union_schema, settings, naming, extra_types),
        Schema::Record(record_schema) => {
            sanitize_container_name(&record_schema.name, &settings.default_namespace)
        }
        Schema::Enum(enum_schema) => sanitize_container_name(&enum_schema.name, &settings.default_namespace),
        Schema::Fixed(fixed_schema) => Ok(format!("[u8; {}]", fixed_schema.size)),
        Schema::Decimal(_) => Ok("apache_avro::Decimal".to_string()),
        Schema::BigDecimal => Ok("apache_avro::BigDecimal".to_string()),
        Schema::Uuid => Ok("Uuid::uuid".to_string()),
        Schema::Date => Ok(datetime_type.to_string()),
        Schema::TimeMillis => Ok(datetime_type.to_string()),
        Schema::TimeMicros => Ok(datetime_type.to_string()),
        Schema::TimestampMillis => Ok(datetime_type.to_string()),
        Schema::TimestampMicros => Ok(datetime_type.to_string()),
        Schema::TimestampNanos => Ok(datetime_type.to_string()),
        Schema::LocalTimestampMillis => Ok(datetime_type.to_string()),
        Schema::LocalTimestampMicros => Ok(datetime_type.to_string()),
        Schema::LocalTimestampNanos => Ok(datetime_type.to_string()),
        Schema::Duration => Ok("apache_avro::Duration".to_string()),
        Schema::Ref { name: ref_name } => sanitize_container_name(ref_name, &settings.default_namespace),
    }
}

fn get_field_type_array(
    items_schema: &Schema,
    settings: &ProcessSettings,
    naming: &StructFieldName,
    extra_types: &mut Vec<GeneratedType>,
) -> Result<String> {
    let items_type = get_field_type(items_schema, settings, naming, extra_types)?;

    Ok(format!("Vec<{}>", items_type))
}

fn get_field_type_map(
    items_schema: &Schema,
    settings: &ProcessSettings,
    naming: &StructFieldName,
    extra_types: &mut Vec<GeneratedType>,
) -> Result<String> {
    let items_type = get_field_type(items_schema, settings, naming, extra_types)?;

    Ok(format!("std::collections::HashMap<String, {}>", items_type))
}

pub fn sanitize_container_name(
    full_name: &Name,
    default_namespace: &Option<String>,
) -> Result<String> {
    let default_namespace = match default_namespace {
        None => "".to_string(),
        Some(ns) => ns.replace(".", "::") + "::",
    };

    let namespace = match &full_name.namespace {
        None => "".to_string(),
        Some(ns) => {
            ns.split('.')
                .map(|f| SanitizedName::from_module(f).sanitized_name)
                .collect::<Vec<String>>()
                .join("::")
                + "::"
        }
    };

    let type_sanitized_name = SanitizedName::from_type(full_name.name.as_ref());

    Ok(format!(
        "crate::{default_namespace}{namespace}{}",
        type_sanitized_name.sanitized_name
    ))
}

fn get_field_type_union(
    schema: &UnionSchema,
    settings: &ProcessSettings,
    naming: &StructFieldName,
    extra_types: &mut Vec<GeneratedType>,
) -> Result<String> {
    let allvariants = schema.variants();

    if allvariants.len() == 1 {
        return get_field_type(&allvariants[0], settings, naming, extra_types);
    }

    if allvariants.len() == 2 {
        if let Schema::Null = allvariants[0] {
            return Ok(format!(
                "Option<{}>",
                get_field_type(&allvariants[1], settings, naming, extra_types)?
            ));
        }
        if let Schema::Null = allvariants[1] {
            return Ok(format!(
                "Option<{}>",
                get_field_type(&allvariants[0], settings, naming, extra_types)?
            ));
        }
    }

    // Plusieurs variantes non-null : on génère une enum dédiée `{Struct}{Field}`.
    let has_null = allvariants.iter().any(|v| matches!(v, Schema::Null));

    let mut union_variants = Vec::new();
    for variant_schema in allvariants.iter().filter(|v| !matches!(v, Schema::Null)) {
        let type_name = get_field_type(variant_schema, settings, naming, extra_types)?;
        let variant_name = union_variant_name(variant_schema);
        union_variants.push(GeneratedUnionVariant { variant_name, type_name });
    }

    let generated_union = GeneratedUnion::new(naming.struct_name, naming.field_name, union_variants);
    let union_type_name = generated_union.type_name().to_string();
    extra_types.push(GeneratedType::Union(generated_union));

    if has_null {
        Ok(format!("Option<{}>", union_type_name))
    } else {
        Ok(union_type_name)
    }
}

/// Nom donné à la variante de l'enum générée pour un type d'une union.
pub(crate) fn union_variant_name(schema: &Schema) -> SanitizedName {
    let name = match schema {
        Schema::Null => "Null".to_string(),
        Schema::Boolean => "Boolean".to_string(),
        Schema::Int => "Int".to_string(),
        Schema::Long => "Long".to_string(),
        Schema::Float => "Float".to_string(),
        Schema::Double => "Double".to_string(),
        Schema::Bytes => "Bytes".to_string(),
        Schema::String => "String".to_string(),
        Schema::Array(_) => "Array".to_string(),
        Schema::Map(_) => "Map".to_string(),
        Schema::Union(_) => "Union".to_string(),
        Schema::Record(record_schema) => record_schema.name.name.to_string(),
        Schema::Enum(enum_schema) => enum_schema.name.name.to_string(),
        Schema::Fixed(fixed_schema) => fixed_schema.name.name.to_string(),
        Schema::Ref { name } => name.name.to_string(),
        Schema::Decimal(_) => "Decimal".to_string(),
        Schema::BigDecimal => "BigDecimal".to_string(),
        Schema::Uuid => "Uuid".to_string(),
        Schema::Date => "Date".to_string(),
        Schema::TimeMillis => "TimeMillis".to_string(),
        Schema::TimeMicros => "TimeMicros".to_string(),
        Schema::TimestampMillis => "TimestampMillis".to_string(),
        Schema::TimestampMicros => "TimestampMicros".to_string(),
        Schema::TimestampNanos => "TimestampNanos".to_string(),
        Schema::LocalTimestampMillis => "LocalTimestampMillis".to_string(),
        Schema::LocalTimestampMicros => "LocalTimestampMicros".to_string(),
        Schema::LocalTimestampNanos => "LocalTimestampNanos".to_string(),
        Schema::Duration => "Duration".to_string(),
    };

    SanitizedName::from_type(&name)
}
