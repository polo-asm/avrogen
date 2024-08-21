use avrogen::Avrogen;
use clap::Parser;
pub mod error;

pub use crate::error::Result;
pub use crate::error::AvrogenError;

pub fn main() -> core::result::Result<(),Box<dyn std::error::Error>>{

    let avrogen = Avrogen::parse();
    
    avrogen.execute()?;
    Ok(())
}

