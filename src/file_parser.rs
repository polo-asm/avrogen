use crate::{error::AvrogenError, source::AvroFile, Result};
use apache_avro::Schema;


pub fn parse_schemas(files: Vec<AvroFile>) -> Result<Vec<Schema>> 
{
    let mut schema_list = Vec::<Schema>::new();

    for file in files {

        let schema = Schema::parse_str(&file.content)
        .map_err(|e| AvrogenError::Custom(format!("{:?}: {e}",file.file_path)))?;
        
        log::debug!("schema {} read",file.file_path);

        schema_list.push(schema)
    }

    return Ok(schema_list);
}
