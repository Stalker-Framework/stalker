pub mod model;
mod traits;

pub use traits::{Mutatable, RawMutatable};

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use model::Bitflip;
    use stalker_utils::asm::Asm;
    use stalker_utils::context::PreContext;
    use stalker_utils::fmt::hex;

    #[test]
    fn test_db() -> Result<()> {
        let mut ctx = PreContext::default().data_path("/tmp/stalker").init()?;
        println!("{:?}", ctx.config);
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
