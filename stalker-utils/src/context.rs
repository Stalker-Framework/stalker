use super::asm::{Asm, Result};
use super::config::StalkerConfig;
use super::fmt;
use hex::FromHex;
use rzpipe::RzPipe;
use std::boxed::Box;

pub struct Context {
    pub config: StalkerConfig,
    pub rz: Box<RzPipe>,
    arch_cli_arg: String,
}

impl Default for Context {
    fn default() -> Self {
        let mut rz = RzPipe::spawn("/bin/ls", None).expect("Spawn pipe");
        rz.cmd("aa").expect("Perform analysis");
        let config = StalkerConfig::default();
        let arch_cli_arg = config.arch.to_cli_arg();
        Context {
            config,
            rz: Box::new(rz),
            arch_cli_arg,
        }
    }
}

impl Context {
    pub fn asm(&mut self, disasm: String) -> Result<Asm> {
        let c = format!("rz-asm {} {}", self.arch_cli_arg, disasm);
        let value_s = self.rz.cmd(&c)?;
        let value = Vec::from_hex(value_s)?;
        Ok(Asm {
            size: value.len() as u8,
            bytes: value,
            disasm: Some(disasm),
            mutants: None,
        })
    }

    pub fn disasm(&mut self, value: &[u8]) -> Result<Asm> {
        let c = format!("rz-asm {} -d {:x?}", self.arch_cli_arg, fmt::hex(value));

        let disasm = self.rz.cmd(&c)?;
        let _disasm: Option<String> = if disasm.contains("invalid") {
            None
        } else {
            Some(disasm.replace('\n', ";"))
        };
        Ok(Asm {
            size: value.len() as u8,
            bytes: value.to_vec(),
            disasm: _disasm,
            mutants: None,
        })
    }
}
