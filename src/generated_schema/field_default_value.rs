use apache_avro::schema::UnionSchema;
use apache_avro::Schema;
use serde_json::{Map, Value};
use crate::Result;
use crate::generated_schema::ProcessSettings;
use super::field_type::{get_field_type, is_nullable, union_variant_name, StructFieldName};
use super::schema::union_type_name;

#[derive(Debug)]
pub struct FieldDefault{
    content: String,
}

/// Juste utilisé pour produire un nom de type dans les messages d'erreur ci-dessous:
/// pas de contexte de nommage ni de type généré à enregistrer.
fn describe_field_type(field_schema: &Schema, settings: &ProcessSettings) -> Result<String> {
    let naming = StructFieldName { struct_name: "", field_name: "" };
    let mut ignored_extra_types = Vec::new();
    get_field_type(field_schema, settings, &naming, &mut ignored_extra_types)
}


impl FieldDefault{
    pub fn from(default_value: &serde_json::Value,field_schema: &Schema,settings: &ProcessSettings,naming: &StructFieldName)-> Result<FieldDefault>
    {
        let content= get_field_default_value(default_value,field_schema,settings,naming)?;
        Ok(FieldDefault{ content})
    }

    pub fn write_content(&self) -> Result<String> {
        Ok(self.content.to_owned())
    }
}

fn get_field_default_value(default_value: &serde_json::Value,field_schema: &Schema,settings: &ProcessSettings,naming: &StructFieldName) -> Result<String> {

    // Union à plusieurs variantes non-null: la valeur générée doit être enveloppée
    // dans la variante correspondante de l'enum `{Struct}{Field}`.
    if let Schema::Union(union_schema) = field_schema {
        let non_null_variants_count = union_schema.variants().iter().filter(|v| !matches!(v, Schema::Null)).count();
        if non_null_variants_count > 1 {
            return get_field_default_union_value(default_value, union_schema, settings, naming);
        }
    }

    let mut value_as_string = match default_value {
        serde_json::Value::Null => Ok("None".to_string()),
        serde_json::Value::Bool(bool_val) => Ok(format!("{bool_val}")),
        serde_json::Value::Number(num_val) => Ok(format!("{num_val}")),
        serde_json::Value::String(string_val) => Ok(format!("\"{string_val}\".to_string()")),
        serde_json::Value::Array(array) =>  get_field_default_array_value(array,field_schema,settings,naming),
        serde_json::Value::Object(object) => get_field_default_object_value(object,field_schema,settings,naming),
    }?;

    // When the type is nullable and the default value is not null => return Some(default)
    if  is_nullable(field_schema) && value_as_string != *"None"
    {
        value_as_string= format!("Some({value_as_string})");
    }
    Ok(value_as_string)
}

/// Le standard Avro impose que la valeur par défaut d'une union corresponde au
/// type de la première variante déclarée. On génère donc la valeur pour cette
/// première variante, puis on l'enveloppe dans `{Enum}::{Variant}(...)`.
fn get_field_default_union_value(default_value: &serde_json::Value,union_schema: &UnionSchema,settings: &ProcessSettings,naming: &StructFieldName) -> Result<String> {
    let allvariants = union_schema.variants();
    let has_null = allvariants.iter().any(|v| matches!(v, Schema::Null));
    let first_variant = &allvariants[0];

    if let Schema::Null = first_variant {
        return Ok("None".to_string());
    }

    let variant_value = get_field_default_value(default_value, first_variant, settings, naming)?;
    let union_type_name = union_type_name(naming.struct_name, naming.field_name).sanitized_name;
    let variant_name = union_variant_name(first_variant).sanitized_name;

    let wrapped = format!("{union_type_name}::{variant_name}({variant_value})");

    if has_null {
        Ok(format!("Some({wrapped})"))
    } else {
        Ok(wrapped)
    }
}

fn get_field_default_array_value(values_map: &[Value],field_schema: &Schema,settings: &ProcessSettings,naming: &StructFieldName) -> Result<String>{
    match field_schema {
        Schema::Array(inner_type) => {
            if values_map.is_empty(){
                Ok("Vec::new()".to_string())
            }
            else {
        
                let values_joined=values_map
                .iter()
                .map(|v|get_field_default_value(v,&inner_type.items,settings,naming).unwrap())
                .collect::<Vec<String>>()
                .join(", ");
        
                Ok(format!("vec![{values_joined}]"))
            }
        }
        _ =>
        {
            // No need to send Namespace, it's just for logs...
            let field_type = describe_field_type(field_schema,settings)?;
            Err(format!("Impossible to manage default value Array for type which is a {}",field_type).into())
        }
    }

}

fn get_field_default_object_value(values_map: &Map<String, Value>,field_schema: &Schema,settings: &ProcessSettings,naming: &StructFieldName) -> Result<String>{
    match field_schema {
        Schema::Map(inner_type) => {
            if values_map.is_empty(){
                Ok("std::collections::HashMap::new()".to_string())
            }
            else {
        
                let values_joined=values_map
                .iter()
                .map(|(key,value)|format!("(\"{key}\",{})",get_field_default_value(value,&inner_type.types,settings,naming).unwrap()))
                .collect::<Vec<String>>()
                .join(",\n");
        
                Ok(format!("HashMap::from([{values_joined}])"))
            }
        }
        Schema::Record(_record_schema)=> Ok("???".to_string()),
        
        _ =>
        {
            // No need to send Namespace, it's just for logs...
            let field_type = describe_field_type(field_schema,settings)?;
            Err(format!("Impossible to manage default value Object for type which is a {}",field_type).into())
        }
    }

}
