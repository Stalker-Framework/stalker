use super::asm::Asm;
use super::binary::BinaryInfo;
use super::config::Config;
use super::db::Db;
use super::fmt;
use super::Result;
use hex::FromHex;
use rzpipe::RzPipe;
use std::boxed::Box;
use std::process::Command;

pub struct Context {
    pub config: Config,
    pub binary_info: BinaryInfo,
    pub rz: Box<RzPipe>,
    db: Option<Db>,
}

impl Default for Context {
    fn default() -> Self {
        let mut rz = RzPipe::spawn("/bin/ls", None).expect("Spawn pipe");
        rz.cmd("aa").expect("Perform analysis");
        let mut config = Config::default();
        let binary_info_s = rz.cmd("ij").expect("");
        let binary_info: BinaryInfo = serde_json::from_str(&binary_info_s).unwrap();
        config.arch.arch = binary_info.bin.arch.clone();
        config.arch.bits = binary_info.bin.bits;
        Context {
            config,
            rz: Box::new(rz),
            binary_info,
            db: None,
        }
    }
}

impl Context {
    pub fn init_db(&mut self) -> Result<()> {
        if self.db.is_none() {
            let db = Db::new(&self.config.db_path)?;
            self.db = Some(db);
        }
        Ok(())
    }

    pub fn asm(&self, disasm: String) -> Result<Asm> {
        let c = Command::new("rz-asm")
            .args([
                "-a",
                &self.config.arch.arch,
                "-b",
                &self.config.arch.bits.to_string(),
                &disasm,
            ])
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
                &self.config.arch.arch,
                "-b",
                &self.config.arch.bits.to_string(),
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
