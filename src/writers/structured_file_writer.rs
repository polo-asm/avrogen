use std::io::Write;
use std::path::PathBuf;

use crate::generated_schema::namespace::NamespaceInfo;
use crate::Result;
use super::{ensure_folder_exists, create_truncated_file, sorted_children, write_sorted_generated_types};

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

pub fn write_to_structured_files(parent_folder: PathBuf, namespace: NamespaceInfo) -> Result<()> {
    ensure_folder_exists(&parent_folder)?;

    let file_path = module_filename(parent_folder.to_owned(), &namespace);

    if namespace.children.is_empty() && namespace.generated_types.is_empty() {
        return Ok(());
    }

    if !namespace.is_root {
        create_current_file(&namespace, file_path)?;
    }

    for (_, child) in namespace.children.into_iter() {
        let sub_folder: PathBuf = parent_folder
            .clone()
            .join(&namespace.name.sanitized_name);

        write_to_structured_files(sub_folder, child)?;
    }

    Ok(())
}

fn create_current_file(namespace: &NamespaceInfo, file_path: PathBuf) -> Result<()> {
    let mut file = create_truncated_file(file_path)?;

    if !namespace.children.is_empty() {
        for (_, child) in sorted_children(namespace) {
            write!(
                file,
                "pub mod {};\n",
                &child.name.sanitized_name
            )?;
        }

        file.write_all("\n".as_bytes())?;
    }

    write_sorted_generated_types(&mut file, 0, namespace)?;

    Ok(())
}
