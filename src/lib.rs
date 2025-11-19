#![doc = include_str!("../Readme.md")]

use file_parser::parse_schemas;
use generated_schema::namespace::NamespaceInfo;
use std::{path::PathBuf, str::FromStr};

use crate::error::Result;
use clap::Parser;
use clap_verbosity::Verbosity;
use log::{debug, info, LevelFilter};

mod browse_sub_schemas;
mod error;
mod file_parser;
mod generated_schema;
mod source;
mod writers;

#[cfg( feature = "schema_registry")]
mod schema_registry;

/// The Avrogen stucture is the main part of the utility.
/// You need to create an instance of this object and execute it to generate rust files from your avsc files
/// # example
/// ```
/// let builder=avrogen::Avrogen::new()
///          .add_source("schemas/*")
///          .default_namespace("schemas")
///          .output_folder_from_str("src/")
///          .execute();
/// ```

// This program allow allow to generate rust code from avro definition files.
//
// The code generated can be in a modul hierarchy and in the future we will generate a single file module structure.
#[derive(Debug, Parser)]
#[command(version)]
pub struct Avrogen {

    /// Schema registry source.
    #[clap(skip)]
    #[cfg( feature = "schema_registry")]
    schema_registry_source: Option<crate::schema_registry::SchemaRegistrySource>,

    /// Source to use by the generator, use stdin if source isn't specified.
    ///
    /// The source use glob format, you can use multiple source arguments. For simple search: ./MyFolder/*.avsc. For recursive search: ./MyFolder/**/*.avsc
    #[arg(short='s', long, num_args=..)]
    source: Vec<String>,

    /// Allow to define a default namespace for generated code. All namespace generated will be in this default namespace.
    #[arg(short='n', long, aliases=&["namespace", "default_namespace", "default-namespace"])]
    default_namespace: Option<String>,

    #[arg(long, short='o', default_value="./", aliases=&["output-folder","outputfolder"])]
    output_folder: PathBuf,

    #[command(flatten)]
    verbose: Verbosity,

    log_level: Option<LevelFilter>,

    #[arg(long)]
    flat_ouptut: bool
}

impl Default for Avrogen {
    fn default() -> Self {
        Avrogen::new()
    }
}

impl Avrogen {
    /// Create a new Avrogen instance
    /// # example
    /// ```
    /// let builder=avrogen::Avrogen::new();
    /// builder.add_source("folder/*").execute();
    /// ```
    pub fn new() -> Self {
        Avrogen {
            source: vec![],
            default_namespace: None,
            output_folder: PathBuf::from("./"),
            verbose: Verbosity::default(),
            log_level: None,
            flat_ouptut: false,
            #[cfg( feature = "schema_registry")]
            schema_registry_source: None,
        }
    }

    /// For builder syntax, allow to append multiple sources
    /// # example
    /// ```
    /// let builder=avrogen::Avrogen::new();
    /// builder.add_sources(vec!["folder1/*.avsc".to_string(),"folder2/**/*.avsc".to_string()]);
    /// ```
    pub fn add_sources(mut self, mut file_patterns: Vec<String>) -> Self {
        self.source.append(&mut file_patterns);
        self
    }

    /// For builder syntax, allow to add a source
    /// # example
    /// ```
    /// let builder=avrogen::Avrogen::new();
    /// builder.add_source("folder1/*.avsc");
    /// ```
    pub fn add_source(mut self, file_pattern: &str) -> Self {
        self.source.push(file_pattern.to_string());
        self
    }

    #[cfg( feature = "schema_registry")]
    /// For builder syntax, allow to add a Schema registry source
    /// # example
    /// ```
    /// let builder=avrogen::Avrogen::new();
    /// builder.add_schema_registry_source("http://my-schema-registry:8080/").add_subject("montopic-value");
    /// ```
    pub fn add_schema_registry_source(mut self, schema_registry_source: &str) -> Self {
        self.schema_registry_source= Some(crate::schema_registry::SchemaRegistrySource::new(schema_registry_source.to_string()));
        self
    }

    #[cfg( feature = "schema_registry")]
    /// For builder syntax, allow to add a Subject to the schema registry source
    /// # example
    /// ```
    /// let builder=avrogen::Avrogen::new();
    /// builder.add_schema_registry_source("http://my-schema-registry:8080/").add_subject("mytopic-value");
    /// ```
    pub fn add_subject(mut self, subject: &str) -> Self {
        if let Some(schema_registry_source) = self.schema_registry_source.as_mut() {
            schema_registry_source.add_subject(subject);
        }
        else {
            log::error!("add_schema_registry_source must be called first!");
        }
        self
    }

    /// For builder syntax, allow to specify default namespace (or module in Rust)
    /// # example
    /// ```
    /// let builder=avrogen::Avrogen::new();
    /// builder.default_namespace("com.monsite");
    /// ```
    pub fn default_namespace(mut self, default_namespace: &str) -> Self {
        self.default_namespace = Some(default_namespace.to_string());
        self
    }

    /// For builder syntax, allow to specify output folder
    /// # example
    /// ```
    /// let builder=avrogen::Avrogen::new();
    /// builder.output_folder(std::path::PathBuf::from("MyFolder"));
    /// ```
    pub fn output_folder(mut self, output_folder: PathBuf) -> Self {
        self.output_folder = output_folder;
        self
    }

    /// For builder syntax, allow to specify output folder
    /// # example
    /// ```
    /// let builder=avrogen::Avrogen::new();
    /// builder.output_folder_from_str("MyFolder");
    /// ```
    pub fn output_folder_from_str(mut self, output_folder: &str) -> Self {
        // Unwrap error because seems to be infaillible
        self.output_folder = PathBuf::from_str(output_folder).unwrap();
        self
    }

    /// For builder syntax, allow to specify verbosity to Off
    /// # example
    /// ```
    /// let builder=avrogen::Avrogen::new();
    /// builder.set_verbosity_off();
    /// ```
    pub fn set_verbosity_off(mut self) -> Self {
        self.log_level = Some(LevelFilter::Off);
        self
    }

    /// For builder syntax, allow to specify verbosity to Debug
    /// # example
    /// ```
    /// let builder=avrogen::Avrogen::new();
    /// builder.set_verbosity_debug();
    /// ```
    pub fn set_verbosity_debug(mut self) -> Self {
        self.log_level = Some(LevelFilter::Debug);
        self
    }

    /// For builder syntax, allow to specify that output will be all in one file
    /// # example
    /// ```
    /// let builder=avrogen::Avrogen::new();
    /// builder.set_flat_output();
    /// ```
    pub fn set_flat_output(mut self) -> Self {
        self.flat_ouptut= true;
        self
    }

    /// For builder syntax, allow to specify verbosity to Information
    /// # example
    /// ```
    /// let builder=avrogen::Avrogen::new();
    /// builder.set_verbosity_info();
    /// ```
    pub fn set_verbosity_info(mut self) -> Self {
        self.log_level = Some(LevelFilter::Info);
        self
    }

    /// For builder syntax, allow to specify verbosity to Warning
    /// # example
    /// ```
    /// let builder=avrogen::Avrogen::new();
    /// builder.set_verbosity_warn();
    /// ```
    pub fn set_verbosity_warn(mut self) -> Self {
        self.log_level = Some(LevelFilter::Warn);
        self
    }

    /// For builder syntax, allow to specify verbosity to Error
    /// # example
    /// ```
    /// let builder=avrogen::Avrogen::new();
    /// builder.set_verbosity_error();
    /// ```
    pub fn set_verbosity_error(mut self) -> Self {
        self.log_level = Some(LevelFilter::Error);
        self
    }

    /// For builder syntax, allow to specify verbosity
    /// # example
    /// ```
    /// let builder=avrogen::Avrogen::new();
    /// builder.verbosity(log::LevelFilter::Error);
    /// ```
    pub fn verbosity(mut self, level_filter: log::LevelFilter) -> Self {
        self.log_level = Some(level_filter);
        self
    }

    fn log_level(&self) -> LevelFilter {
        if let Some(x) = self.log_level {
            return x;
        }
        self.verbose.log_level_filter()
    }

    /// For builder syntax, allow to specify verbosity
    /// # example
    /// ```
    /// let builder=avrogen::Avrogen::new();
    /// builder.add_source("folder/*").execute();
    /// ```
    pub fn execute(self) -> Result<()> {
        let mut builder = colog::basic_builder();
        builder.filter(None, self.log_level());

        let _ = builder.try_init();

        info!("1) Browse source to get content");

        // We get a list of string. Each string is the content of a file.
        #[warn(unused_mut)]
        let mut file_contents = source::read_files(self.source)?;

        #[cfg( feature = "schema_registry")]
        if let Some(schema_registry_source) = self.schema_registry_source {

            info!("1.bis) Browse schema registry source");
            let schemas = schema_registry_source.get_schemas()?;
            file_contents.extend( schemas );
        }

        info!("2) Parsing file to get schemas...");

        let root_schemas = parse_schemas(file_contents)?;

        let mut root_ns = NamespaceInfo::root(self.default_namespace);

        debug!(
            "{} root schemas found, browse sub schemas...",
            root_schemas.len()
        );

        let root_schemas = root_schemas.iter().collect();

        let all_schemas = browse_sub_schemas::all_schemas_to_generate(root_schemas);

        debug!("Total of {} schemas found", all_schemas.len());

        info!("3) Process schemas to get informations...");

        for schema in all_schemas {
            root_ns.process_schema(schema)?;
        }

        info!("4) Write to files");

        writers::write(self.output_folder, root_ns, self.flat_ouptut)?;

        info!("Done!");

        Ok(())
    }
}
