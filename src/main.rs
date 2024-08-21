use avrogen::Avrogen;
use clap::Parser;
pub mod error;

pub fn main() -> core::result::Result<(),Box<dyn std::error::Error>>{

    let avrogen = Avrogen::parse();
    
    avrogen.execute()?;
    Ok(())
}

