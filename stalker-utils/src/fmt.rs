use super::asm::Asm;

pub fn hex(value: &[u8]) -> String {
    let mut s = String::new();
    for b in value {
        s.push_str(&format!("{:02x}", b));
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
