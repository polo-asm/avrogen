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
    subjects: Vec<String>,
}

impl SchemaRegistrySource
{
    pub fn new(url: String) -> Self
    {
        Self{schema_registry_url: url, subjects: Vec::new()}
    }

    pub fn add_subject(&mut self, subject: &str)
    {
        self.subjects.push(subject.to_string());
    }

    pub fn get_schemas(&self) -> Result<Vec<AvroFile>>
    {
        let mut data = Vec::new();

        for subject in self.subjects.iter() {
            let url = format!("{}/subjects/{}/versions/latest", self.schema_registry_url,subject);
            let result=reqwest::blocking::get(url.to_string())?;

            let subject_data: Subject = result.json()?;

            let schema= subject_data.schema;

            if schema.len() > 0
            {
                data.push(AvroFile{content: schema, file_path: url });
            }
            else {
                println!("Schema returned by {} is empty, this schema will be skipped",url);
            }
        }

        Ok(data)
    }
}