use std::{collections::HashMap, fmt::Debug};
use apache_avro::{schema::Name, Schema};
use log::debug;
use crate::Result;

use super::{global::SanitizedName, schema::*};

#[derive(Debug)]
pub struct NamespaceInfo
{
    pub is_root: bool,
    pub name: SanitizedName,
    pub generated_types: HashMap<String,GeneratedType>,
    pub children: HashMap<String,NamespaceInfo>,
    pub default_namespace: Option<String>,
}

impl NamespaceInfo
{    
    pub fn root(default_namespace: Option<String>) ->  Self
    {
        NamespaceInfo{ 
            is_root: true,
            name: SanitizedName::from_module(&"".to_string()),
            children: HashMap::new(),
            generated_types: HashMap::new(),
            default_namespace:default_namespace.clone()
        }
    }
    
    fn new(parent: &NamespaceInfo, ns_begining: &str) -> Self {
        NamespaceInfo{ 
            name: SanitizedName::from_module(&ns_begining), 
            children: HashMap::new(),
            is_root: false,
            generated_types: HashMap::new(),
            default_namespace:parent.default_namespace.clone()
        }
    }

    pub fn process_schema(&mut self,schema: &Schema) -> Result<()>
    {   
        let full_namespace = [self.default_namespace.to_owned(), schema.namespace().to_owned()]
        .into_iter()
        .flatten()
        .collect::<Vec<String>>()
        .join(".");
    
        self.child_process_schema(schema,full_namespace)
    }


     fn child_process_schema(&mut self,schema: &Schema,sub_schema_path: String )-> Result<()>
     {
        if sub_schema_path == ""
        {
            return self.real_process(schema);
        }

        let (ns_begining,ns_endsection) = sub_schema_path.split_once(".").unwrap_or((sub_schema_path.as_str(),""));

        match self.children.get_mut(ns_begining){
            Some(child) => child.child_process_schema(schema, ns_endsection.to_string()),
            None => {
                let mut new_ns = NamespaceInfo::new(&self,ns_begining);

                debug!("New namespace: {:?}", new_ns.name.sanitized_name);


                let result=new_ns.child_process_schema(schema, ns_endsection.to_string());

                self.children.insert(ns_begining.to_string(), new_ns);

                result
            }
        }
     }

     fn real_process(&mut self,schema: &Schema)-> Result<()>
     {
        let unknown_schema_name=Name::new("Unknown_schema_name").unwrap();
        // A changer
        let content = GeneratedType::generate_schema_struct( schema, &self.default_namespace )
        .map_err(|e|format!("{}: {e}", schema
        .name()
        .unwrap_or(&unknown_schema_name)
        .fullname(None)))?;

        if let GeneratedType::None = content{
        }
        else {
            self.generated_types.insert(content.schema_name(), content );
        }

        Ok(())
     }
     
}

