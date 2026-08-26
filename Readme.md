# Build script usage
If you are not familiar with build scripts you can read the [rust documentation about build scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html)

You can use this crate to generate files in your rust project.
To do that you need to add the crate to your build dependencies in the `Cargo.toml` file.
```toml
[build-dependencies]
avrogen = "0.2.0" # Update if needed
```
Then you need to add a `build.rs` file with this kind of content:
```rust
use avrogen::Avrogen;

fn main(){

    let builder= avrogen::Avrogen::new()
        .add_source("Schemas/*.avsc")
        .output_folder_from_str("src/");

    builder.execute().expect("Impossible to generate rust files from avsc files");

    println!("cargo::rerun-if-changed=Schemas/");
}
```
You can also generate the classes in your target folder using OUT_DIR environment variable.

# Standalone usage

`avrogen --help` show you this help:
```shell
This program allow allow to generate rust code from avro definition files.

The code generated can be in a modul hierarchy and in the future we will generate a single file module structure.

Usage: avrogen [OPTIONS]

Options:
  -s, --source [<SOURCE>...]
          Source to use by the generator, use stdin if source isn't specified.

          The source use glob format, you can use multiple source arguments. For simple search: ./MyFolder/*.avsc. For recursive search: ./MyFolder/**/*.avsc

  -n, --default-namespace <DEFAULT_NAMESPACE>
          Allow to define a default namespace for generated code. All namespace generated will be in this default namespace

  -o, --output-folder <OUTPUT_FOLDER>
          [default: ./]

      --flat-ouptut
          

      --date-library <DATE_LIBRARY>
          Allow to choose which crate is used to generate date/time fields (`chrono` or `jiff`)
          
          [default: chrono]
          [possible values: chrono, jiff]

  -v, --verbose...
          More output per occurrence

  -q, --quiet...
          Less output per occurrence

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```


# Features

This tool generate rust modules and classes from asvc files.
The tool use apache-avro structures you must have this create in your project, the tool has need version > 0.17.0
```shell
cargo add apache-avro
```

## Namespaces

The tool generate the same module structure than the namespace in the avsc files. This allow to have multiple types with the same 

We can add a default namespace (module structue). This allow to generate all code in a SubNamespace.

## Naming conventions

Even if your asvc don't follow the rust naming conventions, the tool will generate files with good naming conventions.
- modules use `snake_case`
- struct and enums use `UpperCamelCase`
- fields use `snake_case`

If a field/struct/module use a Rust reserved keyword a prefix will be added.

Some attributes are added to do the mapping between the asvc name and the sanitized name in the rust code.

## Dates 
```shell
cargo add chrono -F serde
```

You can also generate date fields using the [`jiff`](https://crates.io/crates/jiff) crate instead. Use the `--date-library jiff` CLI option, or the `use_jiff()` builder method:
```rust
let builder= avrogen::Avrogen::new()
    .add_source("Schemas/*.avsc")
    .use_jiff();
```
In that case, ensure that you have added the `jiff` crate (with the `serde` feature) in your project:
```shell
cargo add jiff -F serde
```


## Guids 
This tool generate Guid fields which use `uuid` crate. Ensure that you have added this crate in your project with the command:
```shell
cargo add uuid
```

## Unions

For union there are 3 cases:
- 1 value possible, in this case the field will be generated with the type of the value
- 1 value or null, in this case the field will be generated with an Option<other_type>
- multiple values, in this case the field will have a newly generated enum type with the name of the field and the possible values as variants.

# limitations

* [ ] Flatten the namespace structure if you don't want to have a module structure
* [ ] Dates with no lib