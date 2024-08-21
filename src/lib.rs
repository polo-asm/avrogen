use std::path::PathBuf;
use file_parser::parse_schemas;
use generated_schema::namespace::NamespaceInfo;

use clap::Parser;
use clap_verbosity::Verbosity;
use log::{debug, info};
pub use crate::error::Result;

mod generated_schema;
mod file_parser;
mod browse_sub_schemas;
mod source;
mod writers;
pub mod error;


/// This program allow allow to generate rust code from avro definition files.
/// 
/// The code generated can be in a modul hierarchy and in the future we will generate a single file module structure.
#[derive(Debug, Parser)]
#[command(version)]
pub struct Avrogen {

    /// Source to use by the generator, use stdin if source isn't specified.
    /// 
    /// The source use glob format, you can use multiple source arguments. For simple search: ./MyFolder/*.avsc. For recursive search: ./MyFolder/**/*.avsc
    #[arg(short='s', long, num_args=..)]
    pub source: Vec<String>,

    /// Allow to define a default namespace for generated code. All namespace generated will be in this default namespace.
    #[arg(short='n', long, aliases=&["namespace", "default_namespace", "default-namespace"])]
    pub default_namespace: Option<String>,

    #[arg(long, short='o', default_value="./", aliases=&["output-folder","outputfolder"])]
    pub output_folder: PathBuf,

    #[command(flatten)]
    pub verbose: Verbosity,
}

impl Avrogen{
    pub fn new() -> Self{
        Avrogen{
            source:vec![], 
            default_namespace:None,
            output_folder: PathBuf::from("./"),
            verbose: Verbosity::default(),
        }
    }

    // For builder syntax, allow to specify source
    // example of usage `builder.source(["folder1/*.avsc","folder2/**/*.avsc"]);`
    pub fn source(mut self,mut file_patterns: Vec<String>) -> Self
    {
        self.source.append(&mut file_patterns);
        self
    }

    // For builder syntax, allow to specify default namespace (or module in Rust)
    // example of usage `builder.default_namespace(["com.monsite"]);`
    pub fn default_namespace(mut self,default_namespace: &String) -> Self
    {
        self.default_namespace=Some(default_namespace.to_owned());
        self
    }

    // For builder syntax, allow to specify output folder
    // example of usage `builder.output_folder(PathBuf::from("MyFolder"));`
    pub fn output_folder(mut self,output_folder: PathBuf) -> Self
    {
        self.output_folder=output_folder;
        self
    }

    // For builder syntax, allow to specify verbosity
    // example of usage `builder.verbosity(Verbosity::default());`
    pub fn verbosity(mut self,verbosity: Verbosity) -> Self
    {
        self.verbose=verbosity;
        self
    }

    pub fn execute(self) -> Result<()>
    {
        let mut builder= colog::basic_builder();
        builder.filter(None,self.verbose.log_level_filter());

        builder.init();

        info!("1) Browse source to get content");

        // We get a list of string. Each string is the content of a file.
        let file_contents = source::read_files(self.source)?;

        info!("2) Parsing file to get schemas...");

        let root_schemas = parse_schemas(file_contents)?;

        let mut root_ns = NamespaceInfo::root(self.default_namespace);

        debug!("{} root schemas found, browse sub schemas...",root_schemas.len());

        let root_schemas = root_schemas
        .iter()
        .map(|s|s)
        .collect();

        let all_schemas= browse_sub_schemas::all_schemas_to_generate(root_schemas);

        debug!("Total of {} schemas found",all_schemas.len());

        info!("3) Process schemas to get informations...");


        for schema in all_schemas  {
            root_ns.process_schema(schema)?;
        }
        
        info!("4) Write to files");

        writers::write(self.output_folder, root_ns)?;

        info!("Done!");

        Ok(())
    }
}