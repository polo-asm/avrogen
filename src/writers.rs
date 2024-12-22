use std::path::PathBuf;

use structured_file_writer::write_to_structured_files;
use flat_file_writer::write_to_flat_file;

use crate::generated_schema::namespace::NamespaceInfo;
use crate::Result;

mod structured_file_writer;
mod flat_file_writer;


pub fn write(output_folder: PathBuf,root_namespace: NamespaceInfo,flat_output: bool) -> Result<()>{
    
    if flat_output
    {
        write_to_flat_file(output_folder, root_namespace)?;
    }
    else{
        // Currently only one writer exist
        write_to_structured_files(output_folder, root_namespace)?;
    }
    
    Ok(())
}