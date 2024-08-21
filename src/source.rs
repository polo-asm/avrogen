use std::{ fs, io::Read, path::PathBuf};
use glob::glob;

use crate::Result;

pub struct AvroFile{
    pub content: String,
    pub file_path: String,
}

pub fn read_files(source: Vec<String>)-> Result<Vec<AvroFile>>
{
    if source.is_empty() {
        let content: AvroFile = read_stdin()?;
        return Ok(vec![content]);
    }

    let all_paths: Result<Vec<glob::Paths>>=source
                    .iter()
                    .map(|s|
                        glob(s.as_ref())
                        .map_err(|e|crate::error::AvrogenError::GlobPattern(e))
                    )
                    .collect();

    let file_contents: Result<Vec<AvroFile>>=all_paths?
                .into_iter()
                .flat_map(|f| f.filter_map(glob::GlobResult::ok))
                .map(|f| read_file(f))
                .into_iter()
                .collect();
    
    file_contents
}

 fn read_stdin() -> Result<AvroFile> {

    log::debug!("Standard input will be read");

    let mut stdin_content= String::new();

    std::io::stdin().read_to_string(&mut stdin_content)?;

    Ok(AvroFile{ content: stdin_content, file_path: "<stdin>".to_string()})
}

 fn read_file(file_path: PathBuf)-> Result<AvroFile> {

    log::debug!("Reading file {}",file_path.display());
    
     let file_content=fs::read_to_string(file_path.to_owned())?;

     Ok(AvroFile{content: file_content, file_path: file_path.display().to_string()})
}
