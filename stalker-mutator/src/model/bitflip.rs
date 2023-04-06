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

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use stalker_utils::asm::Asm;
    use stalker_utils::context::Context;
    use stalker_utils::fmt::hex;

    #[test]
    fn test_raw_mutants() -> Result<()> {
        let bytes = vec![0u8, 1u8, 2u8, 3u8];
        let mut len = 0;
        for _ in bytes.raw_mutants::<Bitflip>() {
            len += 1;
        }
        assert_eq!(len, 32);
        Ok(())
    }

    #[test]
    fn test_mutants() -> Result<()> {
        let mut ctx = Context::default();
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

    #[test]
    fn test_db() -> Result<()> {
        let mut ctx = Context::default();
        ctx.config.db_path = String::from("/tmp/stalker");
        ctx.lib.init_locs(&mut ctx.rz)?;
        ctx.init_db()?;
        if let Some(db) = ctx.db {
            let locs = ctx.lib.locs[0..1].to_vec(); // Take 1 for test
            for loc in locs.iter().take(1) {
                // Take 1 for test
                let locinfo = ctx.lib.get_locinfo(&mut ctx.rz, &loc.name);
                if locinfo.is_err() {
                    continue;
                }
                for op in locinfo.unwrap().ops.iter().take(2) {
                    // Take 1 for test
                    let asm = Asm::from(op);
                    if let Ok(Some(_)) = db.mutant.get(format!(
                        "{}_{}_{:02x}",
                        ctx.config.arch,
                        hex(&asm.bytes),
                        asm.size * 8 - 1
                    )) {
                        continue;
                    } else {
                        for (i, mutant) in asm
                            .mutants::<Bitflip>(|bytes| ctx.config.arch.disasm(&bytes))
                            .enumerate()
                        {
                            let m = mutant.unwrap();
                            let key = format!("{}_{}_{:02x}", ctx.config.arch, hex(&asm.bytes), i);
                            let val = format!(
                                "{}_{}",
                                hex(&m.bytes),
                                match &m.disasm {
                                    None => "invalid",
                                    Some(asm) => asm,
                                }
                            );
                            println!("{} {}", &key, &val);
                            db.mutant.insert(key, val.as_str())?;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
