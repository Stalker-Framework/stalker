mod cmp;
mod fmt;
pub mod inst;
mod parser;
pub use parser::AsmParser;

#[derive(Debug, Clone)]
pub struct Asm {
    pub bytes: Vec<u8>,
    pub size: u8,
    pub disasm: Option<String>,
    pub mutants: Option<Vec<Asm>>,
}
