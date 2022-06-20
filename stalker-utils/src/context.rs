use super::asm::{Asm, Result};
use super::config::StalkerConfig;
use super::fmt;
use hex::FromHex;
use rzpipe::RzPipe;
use std::boxed::Box;
use std::process::Command;

pub struct Context {
    pub config: StalkerConfig,
    pub rz: Box<RzPipe>,
    arch_cli_arg: String,
    db: sled::Db,
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
            db: sled::open("../../data/sled.db").unwrap(),
        }
    }
}

impl Context {
    pub fn asm(&self, disasm: String) -> Result<Asm> {
        let c = Command::new("rz-asm")
            .args([&self.arch_cli_arg, &disasm])
            .output();
        let value = Vec::from_hex(String::from_utf8(c?.stdout).unwrap())?;
        Ok(Asm {
            size: value.len() as u8,
            bytes: value,
            disasm: Some(disasm),
            mutants: None,
        })
    }

    pub fn disasm(&self, value: &[u8]) -> Result<Asm> {
        let c = Command::new("rz-asm")
            .args([&self.arch_cli_arg, "-d", &fmt::hex(value)])
            .output();

        let disasm = String::from_utf8(c?.stdout).unwrap();
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
