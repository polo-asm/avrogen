use std::path::PathBuf;
use std::path::Path;
use std::fs::{self, File};
use std::io::Write;
use indent::indent_all_by;
use itertools::Itertools;
use log::debug;

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
        write_to_structured_files(output_folder, root_namespace)?;
    }
    
    Ok(())
}

/// Creates `folder` (and any missing parent) if it doesn't exist yet.
fn ensure_folder_exists(folder: &Path) -> Result<()> {
    if !folder.exists() {
        fs::create_dir_all(folder)?;

        debug!("Folder created: {}", folder.display());
    }

    Ok(())
}

/// Opens (creating/truncating) the file at `path` for writing.
fn create_truncated_file(path: PathBuf) -> Result<File> {
    debug!(
        "Will create file: {}",
        path.clone().into_os_string().into_string().unwrap()
    );

    Ok(File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)?)
}

/// Iterates over `namespace`'s children, sorted by name, for deterministic output.
fn sorted_children(namespace: &NamespaceInfo) -> impl Iterator<Item = (&String, &NamespaceInfo)> {
    namespace.children.iter().sorted_by_key(|n| n.0)
}

/// Writes the content of all types generated for `namespace` (sorted by name), indented by `indent` spaces.
fn write_sorted_generated_types(file: &mut impl Write, indent: usize, namespace: &NamespaceInfo) -> Result<()> {
    for (_, content) in namespace.generated_types.iter().sorted_by_key(|p| p.0) {
        let content = indent_all_by(indent, content.produce_content()?);

        file.write_all(content.as_bytes())?;
    }

    Ok(())
}
