mod cmp;
mod fmt;
pub mod inst;
mod parser;
use hex::FromHex;
pub use parser::AsmParser;
use sled::IVec;

use crate::{config::Arch, context::Context};

#[derive(Debug, Clone)]
pub struct Asm {
    pub bytes: Vec<u8>,
    pub size: u8,
    pub disasm: Option<String>,
}

impl From<String> for Asm {
    fn from(src: String) -> Self {
        let splited = src.split('_').collect::<Vec<&str>>();
        let bytes = Vec::from_hex(splited[0]).unwrap();
        let size = bytes.len() as u8;
        let disasm = if splited[1] == "invalid" {
            None
        } else {
            Some(splited[1].to_string())
        };
        Asm {
            bytes,
            size,
            disasm,
        }
    }
}

impl From<&IVec> for Asm {
    fn from(v: &IVec) -> Self {
        Asm::from(String::from_utf8_lossy(v).to_string())
    }
}

impl From<(&Context, &[u8])> for Asm {
    fn from(v: (&Context, &[u8])) -> Self {
        v.0.config.arch.disasm(v.1).unwrap()
    }
}

impl Asm {
    pub fn key(&self, arch: &Arch, index: u8) -> String {
        format!("{}_{}_{:02x}", arch, super::fmt::hex(&self.bytes), index)
    }
}
