use stalker_utils::tag::Tag;

pub use crate::traits::*;

pub struct Bitflip;

impl FaultModel for Bitflip {
    fn next_mutant(iter: &mut IntoIter<Self>) -> Option<Vec<u8>> {
        if iter.bit.1 == 8 {
            iter.bit.0 += 1;
            (iter.mask, iter.bit.1) = (1, 0);
        }
        if iter.bit.0 == iter.size {
            return None;
        }
        let mut bytes = iter.base.clone();
        bytes[iter.bit.0] ^= iter.mask;
        iter.mask <<= 1;
        iter.bit.1 += 1;
        Some(bytes)
    }
}

impl Tag for Bitflip {
    fn tag() -> String {
        "bitflip".into()
    }

    fn id(&self) -> String {
        Self::tag()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use stalker_utils::asm::Asm;
    use stalker_utils::context::PreContext;

    #[test]
    fn test_bitflip_raw_mutants() -> Result<()> {
        let bytes = vec![0u8, 1u8, 2u8, 3u8];
        let mut len = 0;
        for _ in bytes.raw_mutants::<Bitflip>() {
            len += 1;
        }
        assert_eq!(len, 32);
        Ok(())
    }

    #[test]
    fn test_bitflip_mutants() -> Result<()> {
        let mut ctx = PreContext::default().data_path("/tmp/stalker").init()?;
        ctx.lib.init_locs(&mut ctx.rz)?;
        let mut snames = vec![];
        for _ in 0..ctx.lib.locs.len() {
            snames.push(ctx.lib.locs[0].name.clone());
        }
        for sname in snames.iter().take(2) {
            let locinfo = ctx.lib.get_locinfo(&mut ctx.rz, sname)?;
            for op in locinfo.ops.iter().take(3) {
                let asm = Asm::from(op);
                for mutant in asm
                    .mutants::<Bitflip>(|bytes| ctx.config.arch.disasm(&bytes))
                    .filter_map(|res| res.ok().and_then(|asm| asm.disasm))
                {
                    println!("{:?}", mutant);
                }
            }
        }
        Ok(())
    }
}
