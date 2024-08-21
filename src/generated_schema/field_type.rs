use apache_avro::schema::*;
use crate::Result;

use super::global::SanitizedName;

pub fn is_nullable(schema: &Schema) -> bool
{
    match schema {
        Schema::Null => true,
        Schema::Union(union_schema)=>{
            union_schema.variants().iter()
                .any(|x| is_nullable(x))
        }
        _ => false
    }

}
pub fn get_field_type(schema: &Schema,default_namespace: &Option<String>) -> Result<String> {
    match schema {
        Schema::Null => Ok("null".to_string()),
        Schema::Boolean => Ok("bool".to_string()),
        Schema::Int => Ok("i32".to_string()),
        Schema::Long => Ok("i64".to_string()),
        Schema::Float => Ok("f32".to_string()),
        Schema::Double => Ok("f64".to_string()),
        Schema::Bytes => Ok("Vec<u8>".to_string()),
        Schema::String => Ok("String".to_string()),
        Schema::Array(array_schema) => get_field_type_array(array_schema,default_namespace),
        Schema::Map(map_schema) => get_field_type_map(map_schema,default_namespace),
        Schema::Union(union_schema) => get_field_type_union(union_schema,default_namespace),
        Schema::Record(record_schema) => sanitize_container_name(&record_schema.name,default_namespace),
        Schema::Enum(enum_schema) => sanitize_container_name(&enum_schema.name,default_namespace),
        Schema::Fixed(fixed_schema) => Ok(format!("[u8; {}]",fixed_schema.size)),
        Schema::Decimal(_) => Ok("apache_avro::Decimal".to_string()),
        Schema::Uuid => Ok("Uuid::uuid".to_string()),
        Schema::Date => Ok("chrono::NaiveDateTime".to_string()),
        Schema::TimeMillis => Ok("chrono::NaiveDateTime".to_string()),
        Schema::TimeMicros => Ok("chrono::NaiveDateTime".to_string()),
        Schema::TimestampMillis => Ok("chrono::NaiveDateTime".to_string()),
        Schema::TimestampMicros => Ok("chrono::NaiveDateTime".to_string()),
        Schema::LocalTimestampMillis => Ok("chrono::NaiveDateTime".to_string()),
        Schema::LocalTimestampMicros => Ok("chrono::NaiveDateTime".to_string()),
        Schema::Duration => Ok("apache_avro::Duration".to_string()),
        Schema::Ref { name: ref_name } => sanitize_container_name(&ref_name,default_namespace),
    }
}

fn get_field_type_array(items_schema: &Schema,default_namespace: &Option<String>) -> Result<String> {
    let items_type = get_field_type(items_schema,default_namespace)?;
    return Ok(format!("Vec<{}>",items_type));
 }

 fn get_field_type_map(items_schema: &Schema,default_namespace: &Option<String>) -> Result<String> {
    let items_type = get_field_type(items_schema,default_namespace)?;
    return Ok(format!("std::collections::HashMap<String,{}>",items_type));
 }
  

pub fn sanitize_container_name(full_name: &Name,default_namespace: &Option<String>) -> Result<String>
{
    let default_namespace = match default_namespace {
        None => "".to_string(),
        Some(ns) =>ns.replace(".","::")+"::"
    };

    let namespace  = match &full_name.namespace {
        None => "".to_string(),
        Some(ns) =>ns.split('.')
        .map(|f|SanitizedName::from_module(f).sanitized_name)
        .collect::<Vec<String>>()
        .join("::")+"::"
    };

    let type_sanitized_name = SanitizedName::from_type(full_name.name.as_ref());
    
    Ok(format!("crate::{default_namespace}{namespace}{}",type_sanitized_name.sanitized_name))
}

pub fn get_field_type_union(schema: &UnionSchema,default_namespace: &Option<String>) -> Result<String>
{    
    let allvariants = schema.variants();

    if allvariants.len() == 1 {
        return  get_field_type(&allvariants[0],default_namespace);
    }

    
    if allvariants.len() == 2 {
        match allvariants[0] {
            Schema::Null =>  return Ok(format!("Option<{}>",get_field_type(&allvariants[1],default_namespace)?).to_string()), 
            _ => 
                match allvariants[1] {
                    Schema::Null =>  return Ok(format!("Option<{}>",get_field_type(&allvariants[0],default_namespace)?).to_string()), 
                    _ => return Ok("Union(2) ???".to_string()),     
                },     
        };
        
    }

    return Ok("Union(X) ???".to_string());
}