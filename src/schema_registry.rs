use serde::Deserialize;
use crate::Result;
use crate::source::AvroFile;

#[derive(Debug, Deserialize)]
pub struct Subject{
    schema: String,
}

#[derive(Debug,Clone)]
pub struct SchemaRegistrySource
{
    schema_registry_url: String,
    local_folder: String,
    subjects: Vec<String>,
}

impl SchemaRegistrySource
{
    pub fn new(schema_registry_url: String, local_folder: String) -> Self
    {
        Self{schema_registry_url,
            local_folder,
            subjects: Vec::new(),
        }
    }

    pub fn add_subject(&mut self, subject: &str)
    {
        self.subjects.push(subject.to_string());
    }

    pub fn get_schemas(&self) -> Result<Vec<AvroFile>>
    {
        let mut data = Vec::new();

        std::fs::create_dir_all(&self.local_folder)?;


        for subject in self.subjects.iter() {
            let schema_content = self.get_schema_from_registry(subject);

            let file_path = format!("{}/{}.avsc", self.local_folder, subject);

            match schema_content {
                Err(_) => {
                    println!("cargo:warning=Failed to updated schema {}", subject);

                    let file_content = Self::read_from_file(&file_path);

                    if let Some(content) = file_content {
                        data.push(content);
                    }
                } ,
                Ok(schema_content) => {
                    if schema_content.is_empty() {
                        println!("cargo:warning=Schema for subject {} is empty, it will not be updated.", subject);

                        let file_content = Self::read_from_file(&file_path);

                        if let Some(content) = file_content {
                            data.push(content);
                        }
                    }
                    let file_content = std::fs::read_to_string(&file_path);
                    if file_content.is_err() || file_content.unwrap() != schema_content
                    {
                        std::fs::write(&file_path, &schema_content)?;
                        println!("Updated local schema file {}", file_path);
                    }
                    data.push(AvroFile{content: schema_content, file_path });
                }
            }
        }

        Ok(data)
    }

    fn read_from_file(file_path: &str) -> Option<AvroFile>
    {
        let file_content = std::fs::read_to_string(file_path);

        match file_content {
            Err(e) => {
                println!("cargo:warning=Failed to read local schema file {}: {}", file_path, e);
                None
            },
            Ok(content) => {
                Some(AvroFile{content, file_path: file_path.to_string() })
            }
        }
    }

    fn get_schema_from_registry(&self, subject: &str) -> Result<String>
    {
        let url = format!("{}/subjects/{}/versions/latest", self.schema_registry_url,subject);
        let result=reqwest::blocking::get(url.to_string())?;

        let subject_data: Subject = result.json()?;

        Ok(subject_data.schema)
    }
}