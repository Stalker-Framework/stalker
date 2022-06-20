mod cmp;
mod error;
mod fmt;
pub mod inst;
mod parser;
pub use parser::AsmParser;
use error::Error;

#[derive(Debug, Clone)]
pub struct Asm {
  pub bytes: Vec<u8>,
  pub size: u8,
  pub disasm: Option<String>,
  pub mutants: Option<Vec<Asm>>,
}

pub type Result<T, E = Error> = core::result::Result<T, E>;
