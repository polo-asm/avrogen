use std::path::PathBuf;
use log::debug;
use std::fs::File;
use std::io::Write;
use crate::generated_schema::namespace::NamespaceInfo;
use crate::Result;
use super::{ensure_folder_exists, create_truncated_file, sorted_children, write_sorted_generated_types};

pub fn write_to_flat_file(parent_folder: PathBuf, namespace: NamespaceInfo) -> Result<()> {

    // Create the file of the root namespace
    let file =create_file(parent_folder,&namespace)?;

    let indent = 0;

    write_namespace(&file, indent, &namespace)?;

    Ok(())
} 

fn create_file(parent_folder: PathBuf, namespace: &NamespaceInfo) -> Result<File> {

    ensure_folder_exists(&parent_folder)?;

    let file_path = parent_folder
        .join("mod")
        .with_extension("rs");        
    debug!("namespace name: {}", &namespace.name.sanitized_name);

    create_truncated_file(file_path)
}

fn write_namespace(mut file:&File,indent: usize,namespace: &NamespaceInfo) -> Result<()>
{
    if !namespace.children.is_empty() {
        let indented_prefix= " ".repeat(indent);
        let sub_indent = indent+4;

        for (_, child) in sorted_children(namespace) {
            write!(
                file,
                "{}pub mod {} {{\n\n",
                indented_prefix,
                &child.name.sanitized_name
            )?;

            write_namespace(file, sub_indent, child)?;

            write!(file,"{}}}\n",indented_prefix)?;
        }
    }

    write_sorted_generated_types(&mut file, indent, namespace)?;

    Ok(())
}