use stalker_utils::asm::Asm;
use stalker_utils::context::Context;
use std::boxed::Box;

pub struct AsmMutants {
    base: Vec<u8>,
    size: usize,
    ctx: Box<Context>,
    bit: (usize, usize),
    mask: u8,
}

pub trait Mutatable
where
    Self: Sized,
{
    fn mutants(&self, ctx: Option<Context>) -> AsmMutants;
}

impl Mutatable for Asm {
    fn mutants(&self, ctx: Option<Context>) -> AsmMutants {
        let ctx = ctx.unwrap_or_default();
        AsmMutants {
            base: self.bytes.to_vec(),
            size: self.size as usize,
            ctx: Box::new(ctx),
            bit: (0, 0),
            mask: 1,
        }
    }
}

impl Iterator for AsmMutants {
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
                let _ = asm.meta();
                let _mutants = asm.mutants(Some(Context::default()));
                for mutant in _mutants {
                    println!("{:?}", mutant);
                }
            }
        }
        Ok(())
    }
}
