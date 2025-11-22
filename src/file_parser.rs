use crate::{error::AvrogenError, source::AvroFile, Result};
use apache_avro::Schema;


pub fn parse_schemas(files: Vec<AvroFile>) -> Result<Vec<Schema>> 
{
    let schema_content_list: Vec<&str> = files.iter().map(|f| f.content.as_str()).collect();
    Schema::parse_list(&schema_content_list)
        .map_err(|e| { AvrogenError::Custom(format!("{}",e)) })

}
