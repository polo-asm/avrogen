# Avrogen software

## About

`avrogen --help` show you this help:
```
This program allow allow to generate rust code from avro definition files.

The code generated can be in a modul hierarchy and in the future we will generate a single file module structure.

Usage: avrogen.exe [OPTIONS]

Options:
  -s, --source [<SOURCE>...]
          Source to use by the generator, use stdin if source isn't specified.

          The source use glob format, you can use multiple source arguments. For simple search: ./MyFolder/*.avsc. For recursive search: ./MyFolder/**/*.avsc

  -n, --default-namespace <DEFAULT_NAMESPACE>
          Allow to define a default namespace for generated code. All namespace generated will be in this default namespace

  -o, --output-folder <OUTPUT_FOLDER>
          [default: ./]

  -v, --verbose...
          More output per occurrence

  -q, --quiet...
          Less output per occurrence

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
## Features

This tool generate rust modules and classes from asvc files.

### Namespaces

The tool generate the same module structure than the namespace in the avsc files. This allow to have multiple types with the same 

We can add a default namespace (module structue). This allow to generate all code in a SubNamespace.

### Naming conventions

Even if your asvc don't follow the rust naming conventions, the tool will generate files with good naming conventions.
- modules use `snake_case`
- struct and enums use `UpperCamelCase`
- fields use `snake_case`

If a field/struct/module use a Rust reserved keyword a prefix will be added.

Some attributes are added to do the mapping between the asvc name and the sanitized name in the rust code.

### Dates 
This tool generate date fields which use `chrono` crate. Ensure that you have added this crate in your project with the command `cargo add chrono -F serde`

### Guids 
This tool generate Guid fields which use `uuid` crate. Ensure that you have added this crate in your project with the command `cargo add uuid`

## limitations

- [] Multiple union are not well managed.
- [] Flatten the namespace structure if you don't want to have a module structure
- [] Dates without chrono
- [] Save to one file only
- [] Save to stdout