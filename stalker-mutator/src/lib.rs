use stalker_utils::asm::Asm;
use stalker_utils::context::Context;

pub struct AsmMutants<'a> {
    ctx: &'a Context,
    base: Vec<u8>,
    size: usize,
    bit: (usize, usize),
    mask: u8,
}

pub trait Mutatable<'a>
where
    Self: Sized,
{
    fn mutants(&self, ctx: &'a Context) -> AsmMutants<'a>;
}

impl<'a> Mutatable<'a> for Asm {
    fn mutants(&self, ctx: &'a Context) -> AsmMutants<'a> {
        AsmMutants {
            base: self.bytes.to_vec(),
            size: self.size as usize,
            ctx: ctx,
            bit: (0, 0),
            mask: 1,
        }
    }
}

impl<'a> Iterator for AsmMutants<'a> {
    type Item = Asm;
    fn next(&mut self) -> Option<Self::Item> {
        if self.bit.1 == 8 {
            self.bit.0 += 1;
            (self.mask, self.bit.1) = (1, 0);
        }
        if self.bit.0 == self.size {
            return None;
        }
        let mut bytes = self.base.clone();
        bytes[self.bit.0] ^= self.mask;
        let mutant = self.ctx.disasm(&bytes).expect("Failed at creating mutant.");
        self.mask <<= 1;
        self.bit.1 += 1;
        Some(mutant)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use stalker_utils::loc::LibInstance;

    #[test]
    fn test_mutants() -> Result<()> {
        let mut lib = LibInstance::default();
        lib.init_locs()?;
        let mut snames = vec![];
        for _ in 0..lib.locs.len() {
            snames.push(lib.locs[0].name.clone());
        }
        for sname in snames.iter().take(1) {
            let locinfo = lib.get_locinfo(sname)?;
            for op in locinfo.ops.iter().take(1) {
                let asm = Asm::from(op);
                for mutant in asm.mutants(&lib.ctx) {
                    println!("{}", mutant);
                }
            }
        }
        Ok(())
    }
}
