use super::asm::Asm;
use super::fmt;
use super::Result;
use hex::FromHex;
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Arch {
    pub arch: String,
    pub bits: u8,
}

impl Default for Arch {
    fn default() -> Self {
        Arch {
            arch: "arm".into(),
            bits: 64,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Config {
    pub arch: Arch,
    pub os: String,
    pub db_path: String,
    pub rz_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            arch: Arch::default(),
            os: "linux".into(),
            db_path: "data/stalker".into(),
            rz_path: "data/stalker/rizin".into(),
        }
    }
}

impl Arch {
    pub fn asm(&self, disasm: String) -> Result<Asm> {
        let c = Command::new("rz-asm")
            .args(["-a", &self.arch, "-b", &self.bits.to_string(), &disasm])
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
            .args([
                "-a",
                &self.arch,
                "-b",
                &self.bits.to_string(),
                "-d",
                &fmt::hex(value),
            ])
            .output();

        let disasm = String::from_utf8(c?.stdout).unwrap();
        let _disasm: Option<String> = if disasm.is_empty() || disasm.contains("invalid") {
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(default)]
pub struct LibConfig {
    pub path: String,
    pub link_name: String,
    pub syms: SymConfig,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum SymConfig {
    Filter(String),
    Matches(Vec<String>),
}

impl Default for SymConfig {
    fn default() -> Self {
        SymConfig::Filter(String::from("aes"))
    }
}

impl Default for LibConfig {
    fn default() -> Self {
        LibConfig {
            path: String::from("../cryptolibs/dist/libgcrypt/lib/libgcrypt.so.20.3.4"),
            link_name: String::from("libgcrypt.so"),
            syms: Default::default(),
        }
    }
}

impl SymConfig {
    pub fn predicate<'a>(&'a self) -> Box<dyn Fn(&str) -> bool + 'a> {
        match self {
            SymConfig::Filter(target) => Box::new(move |x: &str| x.contains(target)),
            SymConfig::Matches(targets) => {
                Box::new(move |x: &str| targets.into_iter().any(|t| t == x))
            }
        }
    }
}
