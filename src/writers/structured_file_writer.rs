use itertools::Itertools;
use log::debug;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use crate::generated_schema::namespace::NamespaceInfo;
use crate::Result;

/*
Return the module file name. This module can contains:
- the submodule declaration if there is some sub namespaces
- the class contents
*/
fn module_filename(parent_folder: PathBuf, namespace: &NamespaceInfo) -> PathBuf {
    return parent_folder
        .join(namespace.name.sanitized_name.to_string())
        .with_extension("rs");
}

pub fn write_to_structured_files(parent_folder: PathBuf, namespace: NamespaceInfo) -> Result<()> {
    if !parent_folder.exists() {
        fs::create_dir_all(parent_folder.as_path())?;

        debug!("Folder created: {}", parent_folder.display());
    }

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
            .join(namespace.name.sanitized_name.to_string());

        write_to_structured_files(sub_folder, child)?;
    }

    Ok(())
}

fn create_current_file(namespace: &NamespaceInfo, file_path: PathBuf) -> Result<()> {
    debug!(
        "Will create file: {}",
        file_path.clone().into_os_string().into_string().unwrap()
    );

    let mut file = File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file_path)?;

    if !namespace.children.is_empty() {
        for (_, child) in namespace.children.iter() {
            write!(
                file,
                "pub mod {};\r\n",
                child.name.sanitized_name.to_string()
            )?;
        }

        file.write("\r\n".as_bytes())?;
    }

    for (_, content) in namespace.generated_types.iter().sorted_by_key(|p| p.0) {
        file.write(content.produce_content()?.as_bytes())?;
    }

    Ok(())
}
