use std::path::PathBuf;
use indent::indent_all_by;
use itertools::Itertools;
use log::debug;
use std::fs::{self, File};
use std::io::Write;
use crate::generated_schema::namespace::NamespaceInfo;
use crate::Result;

pub fn write_to_flat_file(parent_folder: PathBuf, namespace: NamespaceInfo) -> Result<()> {

    // Create the file of the root namespace
    let file =create_file(parent_folder,&namespace)?;

    let indent = 0;

    write_namespace(&file, indent, &namespace)?;

    Ok(())
} 

fn create_file(parent_folder: PathBuf, namespace: &NamespaceInfo) -> Result<File> {

    if !parent_folder.exists() {
        fs::create_dir_all(parent_folder.as_path())?;

        debug!("Folder created: {}", parent_folder.display());
    }

    //let file_path = parent_folder
    //    .join(&namespace.name.sanitized_name)
    //    .with_extension("rs");

    let file_path = parent_folder
        .join("mod")
        .with_extension("rs");        
    debug!("namespace name: {}", &namespace.name.sanitized_name);
    
    debug!(
        "Will create file: {}",
        file_path.clone().into_os_string().into_string().unwrap()
    );

    let file = File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file_path)?;

    Ok(file)
}

fn write_namespace(mut file:&File,indent: usize,namespace: &NamespaceInfo) -> Result<()>
{
    if !namespace.children.is_empty() {
        let indented_prefix= " ".repeat(indent);
        let sub_indent = indent+4;

        for (_, child) in namespace.children.iter() {
            write!(
                file,
                "{}pub mod {} {{\r\n\r\n",
                indented_prefix,
                &child.name.sanitized_name
            )?;

            write_namespace(file, sub_indent, child)?;

            write!(file,"{}}}\r\n",indented_prefix)?;
        }
    }

    for (_, content) in namespace.generated_types.iter().sorted_by_key(|p| p.0) {
        
        let content=content.produce_content()?;
        
        let content= indent_all_by(indent, content);

        file.write_all(content.as_bytes())?;
    }

    Ok(())
}