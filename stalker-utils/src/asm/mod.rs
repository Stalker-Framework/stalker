use crate::context::Context;
use anyhow::Result;
use hex::FromHex;
mod fmt;
pub mod inst;
mod parser;
pub use parser::AsmParser;

#[derive(Debug)]
pub struct Asm {
  pub bytes: Vec<u8>,
  pub size: u8,
  pub disasm: Option<String>,
  pub mutants: Option<Vec<Asm>>,
}

impl Asm {
  pub fn meta(&self) -> Option<String> {
    let disasm = self.disasm.as_ref()?.to_string();
    let res = AsmParser::parse(&disasm);
    println!("{}", res.unwrap());
    Some(disasm)
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
    let c = format!("rz-asm {} -d {:x?}", args, Context::display_hex(&value));
    let disasm = self.rz.cmd(&c)?;
    let _disasm: Option<String>;
    if disasm.contains("invalid") {
      _disasm = None;
    } else {
      _disasm = Some(disasm.replace("\n", ";").to_string());
    }
    Ok(Asm {
      size: value.len() as u8,
      bytes: value.to_vec(),
      disasm: _disasm,
      mutants: None,
    })
  }
}
