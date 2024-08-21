use crate::Result;
use std::fmt::Write;

pub const RESERVED_NAMES: &'static [&str] = &[
  "as",
  "use",
  "extern crate",
  "break",
  "const",
  "continue",
  "crate",
  "else",
  "if",
  "if let",
  "enum",
  "extern",
  "false",
  "fn",
  "for",
  "if",
  "impl",
  "in",
  "for",
  "let",
  "loop",
  "match",
  "mod",
  "move",
  "mut",
  "pub",
  "impl",
  "ref",
  "return",
  "Self",
  "self",
  "static",
  "struct",
  "super",
  "trait",
  "true",
  "type",
  "unsafe",
  "use",
  "where",
  "while",
  "abstract",
  "alignof",
  "become",
  "box",
  "do",
  "final",
  "macro",
  "offsetof",
  "override",
  "priv",
  "proc",
  "pure",
  "sizeof",
  "typeof",
  "unsized",
  "virtual",
  "yield",
  "dyn", "try", "async", "await", "union" ];


  pub fn format_doc(avro_doc: &Option<String>,spaces: &str) -> Result<String> {
    let mut field_doc= String::from("");
    if let Some(doc) = avro_doc{

        for line in doc.lines() {
            write!(field_doc,"{}/// {}\r\n",spaces,line)?;
        }
    }
    Ok(field_doc)
}

#[derive(Debug)]
pub struct SanitizedName {
    pub sanitized_name: String,
    pub original_name: String,
    pub is_sanitized: bool,
}

impl SanitizedName{
    pub fn from_field(original_name: &str) -> SanitizedName {
        Self::from(original_name, heck::ToSnekCase::to_snek_case,"field_")
    }

    pub fn from_module(original_name: &str) -> SanitizedName {
        Self::from(original_name, heck::ToSnekCase::to_snek_case,"module_")
    }

    pub fn from_type(original_name: &str) -> SanitizedName {
        Self::from(original_name, heck::ToUpperCamelCase::to_upper_camel_case,"Type")
    }

    fn from(original_name: &str,apply_fn:impl Fn(&str) ->String,reserved_name_prefix: &str) -> SanitizedName
    {
        let mut sanitized_name = apply_fn(original_name);

        if RESERVED_NAMES.iter().any(|s| s.to_string()  == sanitized_name) {
            sanitized_name = format!("{}{}",reserved_name_prefix,sanitized_name);
        }
        
        let is_sanitized= sanitized_name != original_name;
        SanitizedName{
            sanitized_name,
            original_name: original_name.to_string(),
            is_sanitized}
    }
}