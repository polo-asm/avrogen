use std::{ fs, io::Read, path::PathBuf};
use glob::glob;

use crate::Result;

pub struct AvroFile{
    pub content: String,
    #[allow(dead_code)]
    pub file_path: String,
}

pub fn read_files(source: Vec<String>)-> Result<Vec<AvroFile>>
{
    if source.is_empty() {
        let content = read_stdin()?;

        return Ok (content.into_iter().collect());
    }

    let all_paths: Result<Vec<glob::Paths>>=source
                    .iter()
                    .map(|s|
                        glob(s.as_ref())
                        .map_err(crate::error::AvrogenError::GlobPattern)
                    )
                    .collect();

    let file_contents: Result<Vec<AvroFile>>=all_paths?
                .into_iter()
                .flat_map(|f| f.filter_map(glob::GlobResult::ok))
                .map(read_file)
                .collect();
    
    file_contents
}

 fn read_stdin() -> Result<Option<AvroFile>> {

    log::debug!("Standard input will be read");

    let mut stdin_content= String::new();

    std::io::stdin().read_to_string(&mut stdin_content)?;

     if stdin_content.is_empty()  {
         Ok(None)
     }
     else {
         Ok(Some(AvroFile{ content: stdin_content, file_path: "<stdin>".to_string()}))
     }
}

 fn read_file(file_path: PathBuf)-> Result<AvroFile> {

    log::debug!("Reading file {}",file_path.display());
    
     let file_content=fs::read_to_string(&file_path)?;

     Ok(AvroFile{content: file_content, file_path: file_path.display().to_string()})
}
