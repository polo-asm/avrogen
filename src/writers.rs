use std::path::PathBuf;

use structured_file_writer::write_to_structured_files;

use crate::generated_schema::namespace::NamespaceInfo;
use crate::Result;

mod structured_file_writer;


pub fn write(output_folder: PathBuf,root_namespace: NamespaceInfo) -> Result<()>{
    
    // Currently only one writer exist
    write_to_structured_files(output_folder, root_namespace)?;
    
    Ok(())
}