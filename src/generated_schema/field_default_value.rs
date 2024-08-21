use apache_avro::Schema;
use serde_json::{Map, Value};
use crate::Result;
use super::field_type::{get_field_type, is_nullable};

#[derive(Debug)]
pub struct FieldDefault{
    content: String,
}

impl FieldDefault{
    pub fn from(default_value: &serde_json::Value,field_schema: &Schema)-> Result<FieldDefault>
    {
        let content= get_field_default_value(default_value,field_schema)?;
        Ok(FieldDefault{ content})
    }

    pub fn write_content(&self) -> Result<String> {
        return Ok(self.content.to_owned())
    }
}

fn get_field_default_value(default_value: &serde_json::Value,field_schema: &Schema) -> Result<String> {
    
    let mut value_as_string = match default_value {
        serde_json::Value::Null => Ok("None".to_string()),
        serde_json::Value::Bool(bool_val) => Ok(format!("{bool_val}")),
        serde_json::Value::Number(num_val) => Ok(format!("{num_val}")),
        serde_json::Value::String(string_val) => Ok(format!("\"{string_val}\".to_string()")),
        serde_json::Value::Array(array) =>  get_field_default_array_value(array,&field_schema),
        serde_json::Value::Object(object) => get_field_default_object_value(object,&field_schema),
    }?;

    // When the type is nullable and the default value is not null => return Some(default)
    if  is_nullable(&field_schema) && value_as_string != "None".to_string()
    {
        value_as_string= format!("Some({value_as_string})");
    }
    Ok(value_as_string)
}

fn get_field_default_array_value(values_map: &Vec<Value>,field_schema: &Schema) -> Result<String>{
    match field_schema {
        Schema::Array(inner_type) => {
            if values_map.is_empty(){
                Ok("Vec::new()".to_string())
            }
            else {
        
                let values_joined=values_map
                .iter()
                .map(|v|get_field_default_value(v,&inner_type).unwrap())
                .collect::<Vec<String>>()
                .join(", ");
        
                Ok(format!("vec![{values_joined}]"))
            }
        }
        _ =>
        {
            // No need to send Namespace, it's just for logs...
            let field_type = get_field_type(&field_schema,&None)?;
            Err(format!("Impossible to manage default value Array for type which is a {}",field_type).into())
        }
    }

}

fn get_field_default_object_value(values_map: &Map<String, Value>,field_schema: &Schema) -> Result<String>{
    match field_schema {
        Schema::Map(inner_type) => {
            if values_map.is_empty(){
                Ok("std::collections::HashMap::new()".to_string())
            }
            else {
        
                let values_joined=values_map
                .iter()
                .map(|(key,value)|format!("(\"{key}\",{})",get_field_default_value(value,inner_type).unwrap()))
                .collect::<Vec<String>>()
                .join(",\r\n");
        
                Ok(format!("HashMap::from([{values_joined}])"))
            }
        }
        Schema::Record(_record_schema)=> Ok("???".to_string()),
        
        _ =>
        {
            // No need to send Namespace, it's just for logs...
            let field_type = get_field_type(&field_schema,&None)?;
            Err(format!("Impossible to manage default value Object for type which is a {}",field_type).into())
        }
    }

}
