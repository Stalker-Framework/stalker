use stalker_utils::asm::Asm;
use stalker_utils::context::Context;

pub trait Mutatable
where
    Self: Sized,
{
    fn mutants(&self, ctx: Option<Context>) -> Vec<Self>;
}

impl Mutatable for Asm {
    fn mutants(&self, ctx: Option<Context>) -> Vec<Self> {
        let mut mutants = vec![];
        if let Some(mut ctx) = ctx {
            let args = ctx.config.arch.to_cli_args();
            for i in 0..self.size {
                let mut mask: u8 = 1;
                for _ in 0..8 {
                    let mut bytes = self.bytes.to_vec();
                    bytes[i as usize] ^= mask;
                    let mutant = ctx
                        .disasm(&args, &bytes)
                        .expect("Failed at creating mutant.");
                    println!("{:?}", mutant);
                    mutants.push(mutant);
                    mask <<= 1;
                }
            }
        } else {
            panic!("No context provided.");
        }
        mutants
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
        for sname in snames.iter() {
            let locinfo = lib.get_locinfo(sname)?;
            for op in locinfo.ops.iter() {
                let asm = Asm::from(op);
                // let _ = asm.meta();
                let _mutants = asm.mutants(Some(Context::default()));
            }
        }
        Ok(())
    }
}
