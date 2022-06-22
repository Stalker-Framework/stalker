use super::asm::Asm;
use super::config::Arch;
use std::fmt::Write;

pub fn hex(value: &[u8]) -> String {
    let mut s = String::new();
    for b in value {
        write!(&mut s, "{:02x}", b).expect("Error occurred while trying to write in hex String");
    }
    s
}

impl std::fmt::Display for Asm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ASM {{ value: ",)?;
        for val in &self.bytes {
            write!(f, "{:02x}", val)?;
        }
        write!(
            f,
            ", size: {}, disasm: {:?}",
            self.size,
            match &self.disasm {
                Some(s) => s,
                None => "invalid",
            }
        )?;
        write!(f, " }}")?;
        Ok(())
    }
}

impl std::fmt::Display for Arch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.arch, self.bits)?;
        Ok(())
    }
}
