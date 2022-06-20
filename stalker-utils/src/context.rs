use super::asm::{Asm, Result};
use super::config::StalkerConfig;
use hex::FromHex;
use rzpipe::RzPipe;
use std::boxed::Box;

pub struct Context {
  pub config: StalkerConfig,
  pub rz: Box<RzPipe>,
}

impl Default for Context {
  fn default() -> Self {
    let mut rz = RzPipe::spawn("/bin/ls", None).expect("Spawn pipe");
    rz.cmd("aa").expect("Perform analysis");
    Context {
      config: StalkerConfig::default(),
      rz: Box::new(rz),
    }
  }
}

impl Context {
  fn display_hex(value: &[u8]) -> String {
    let mut s = String::new();
    for b in value {
      s.push_str(&format!("{:02x}", b));
    }
    s
  }

  pub fn asm(&mut self, args: &str, disasm: String) -> Result<Asm> {
    let c = format!("rz-asm {} {}", args, disasm);
    let value_s = self.rz.cmd(&c)?;
    let value = Vec::from_hex(value_s)?;
    Ok(Asm {
      size: value.len() as u8,
      bytes: value,
      disasm: Some(disasm),
      mutants: None,
    })
  }

  pub fn disasm(&mut self, args: &str, value: &[u8]) -> Result<Asm> {
    let c = format!("rz-asm {} -d {:x?}", args, Context::display_hex(value));
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
