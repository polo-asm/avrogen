use std::path::PathBuf;
use log::debug;
use std::fs::{self, File};
use std::io::Write;
use crate::generated_schema::namespace::NamespaceInfo;
use crate::Result;

/*
Return the module file name. This module can contains:
- the submodule declaration if there is some sub namespaces
- the class contents
*/
fn module_filename(parent_folder: PathBuf, namespace: &NamespaceInfo) -> PathBuf {
    parent_folder
        .join(&namespace.name.sanitized_name)
        .with_extension("rs")
}

pub fn write_to_flat_file(parent_folder: PathBuf, namespace: NamespaceInfo) -> Result<()> {

    // Create the file of the root namespace
    let mut file =create_file(parent_folder,namespace);

    todo!();
} 

fn create_file(parent_folder: PathBuf, namespace: NamespaceInfo) -> Result<File> {

    if !parent_folder.exists() {
        fs::create_dir_all(parent_folder.as_path())?;

        debug!("Folder created: {}", parent_folder.display());
    }

    let file_path = parent_folder
        .join(&namespace.name.sanitized_name)
        .with_extension("rs");
        
    
    debug!(
        "Will create file: {}",
        file_path.clone().into_os_string().into_string().unwrap()
    );

    let mut file = File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file_path)?;

    Ok(file)
}