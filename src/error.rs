use derive_more::From;

pub type Result<T> = core::result::Result<T, AvrogenError>;

#[derive(Debug, From)]
pub enum AvrogenError
{
    #[from]
    Custom(String),

    #[from]
    Io(std::io::Error),

    #[from]
    Fmt(std::fmt::Error),

    #[from]
    Avro(apache_avro::Error),

    #[from]
    GlobPattern(glob::PatternError),

    #[from]
    Glob(glob::GlobError),

}

impl From<&str> for AvrogenError{
    fn from(value: &str) -> Self {
        AvrogenError::Custom(value.to_owned())
    }
}

impl std::fmt::Display for AvrogenError{
  fn fmt(&self,fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error>{
    match self {
        AvrogenError::Custom(e) => write!(fmt,"{e}"),
        AvrogenError::Io(e) => write!(fmt,"{e}"),
        AvrogenError::Fmt(e) => write!(fmt,"{e}"),
        AvrogenError::Avro(e) => write!(fmt,"{e}"),
        AvrogenError::GlobPattern(e) => write!(fmt,"{e}"),
        AvrogenError::Glob(e) => write!(fmt,"{e}"),
    }
 }
}

impl std::error::Error for AvrogenError{}