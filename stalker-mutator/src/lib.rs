use stalker_utils::asm::Asm;

pub struct Mutants<T, F: Fn(&[u8]) -> T> {
    base: Vec<u8>,
    size: usize,
    bit: (usize, usize),
    trans_fn: F,
    mask: u8,
}

pub trait Mutatable
where
    Self: Sized,
{
    fn mutants<T, F: Fn(&[u8]) -> T>(&self, ctx: F) -> Mutants<T, F>;
}

impl Mutatable for Asm {
    fn mutants<T, F: Fn(&[u8]) -> T>(&self, trans_fn: F) -> Mutants<T, F> {
        Mutants {
            base: self.bytes.to_vec(),
            size: self.size as usize,
            bit: (0, 0),
            trans_fn,
            mask: 1,
        }
    }
}

impl Mutatable for Vec<u8> {
    fn mutants<T, F: Fn(&[u8]) -> T>(&self, trans_fn: F) -> Mutants<T, F> {
        Mutants {
            base: self.to_vec(),
            size: self.len(),
            bit: (0, 0),
            trans_fn,
            mask: 1,
        }
    }
}

impl<T, F: Fn(&[u8]) -> T> Iterator for Mutants<T, F> {
    type Item = T;
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
        let mutant = (self.trans_fn)(&bytes);
        self.mask <<= 1;
        self.bit.1 += 1;
        Some(mutant)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use stalker_utils::context::Context;
    use stalker_utils::fmt::hex;

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
            for op in locinfo.ops.iter().take(1) {
                let asm = Asm::from(op);
                for mutant in asm.mutants(|bytes| ctx.config.arch.disasm(bytes)) {
                    println!("{}", mutant.unwrap());
                }
            }
        }
        Ok(())
    }

    #[test]
    fn test_db() -> Result<()> {
        let mut ctx = Context::default();
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
                for op in locinfo.unwrap().ops.iter().take(1) {
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
                            .mutants(|bytes| ctx.config.arch.disasm(bytes))
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

    // #[test]
    // fn test_db_read() -> Result<()> {
    //     let mut ctx = Context::default();
    //     ctx.init_db()?;
    //     if let Some(db) = ctx.db {
    //         let a = db.mutant.get("arm-64_08510318_01")?;
    //         println!("{:?}", a.map(|v| String::from_utf8(v.to_vec())));
    //     }
    //     Ok(())
    // }
}
