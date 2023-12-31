use crate::asm::Asm;
use anyhow::Result;
use rzpipe::RzPipe;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sled::IVec;
use std::convert::From;

#[derive(Default)]
pub struct LibInstance {
    pub locs: Vec<Loc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Loc {
    pub name: String,
    pub realname: Option<String>,
    pub offset: usize,
    pub size: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocInfo {
    pub name: String,
    pub addr: usize,
    pub ops: Vec<LocAsm>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LocAsm {
    #[serde_as(as = "serde_with::hex::Hex")]
    pub bytes: Vec<u8>,
    pub disasm: Option<String>,
    pub offset: usize,
    pub size: u8,
}

impl From<LocAsm> for Asm {
    fn from(asm: LocAsm) -> Asm {
        Asm {
            bytes: asm.bytes,
            disasm: asm.disasm,
            size: asm.size,
        }
    }
}

impl From<&LocAsm> for Asm {
    fn from(asm: &LocAsm) -> Asm {
        Asm {
            bytes: asm.bytes.clone(),
            disasm: asm.disasm.clone(),
            size: asm.size,
        }
    }
}

impl From<(&IVec, Asm)> for LocAsm {
    fn from(t: (&IVec, Asm)) -> LocAsm {
        let offset_str = String::from_utf8_lossy(t.0);
        let offset = usize::from_str_radix(&offset_str, 16).unwrap();
        LocAsm {
            bytes: t.1.bytes.clone(),
            disasm: t.1.disasm.clone(),
            size: t.1.size,
            offset,
        }
    }
}

impl LibInstance {
    pub fn init_locs(&mut self, rz: &mut RzPipe) -> Result<()> {
        if self.locs.is_empty() {
            rz.cmd("fs functions")?;
            let mut f_locs: Vec<Loc> = serde_json::from_str(&rz.cmd("flj")?)?;

            rz.cmd("fs symbols")?;
            let mut s_locs: Vec<Loc> = serde_json::from_str(&rz.cmd("flj")?)?;

            self.locs.append(&mut f_locs);
            self.locs.append(&mut s_locs);
            rz.cmd("fs functions")?;
        }
        Ok(())
    }

    pub fn get_locinfo(&mut self, rz: &mut RzPipe, sname: &str) -> Result<LocInfo> {
        rz.cmd(&format!("s {}", sname))?;
        let mut loc_info: LocInfo = serde_json::from_str(&rz.cmd("pdfj")?)?;
        loc_info.ops = loc_info
            .ops
            .into_iter()
            .filter(|locasm| !locasm.bytes.is_empty())
            .collect::<Vec<LocAsm>>();
        Ok(loc_info)
    }
}
