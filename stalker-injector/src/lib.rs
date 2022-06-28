use sled::Tree;
use stalker_utils::{asm::Asm, config::Arch, context::Context};

pub struct Injection {
    arch: Arch,
    mutant: Tree,
    asm: Option<Asm>,
    locs_iter: sled::Iter,
    index: u8,
}

#[derive(Debug)]
pub struct Change(Asm);

pub trait Injectable {
    fn inject(&self, loc_name: &str) -> Injection;
}

impl Injectable for Context {
    fn inject(&self, loc_name: &str) -> Injection {
        Injection::new(self, loc_name)
    }
}

impl Injection {
    fn new(ctx: &Context, loc_name: &str) -> Self {
        if let Some(db) = &ctx.db {
            let loc_name = loc_name.to_string();
            let mutant = db.mutant.clone();
            let locs = db.instruction.open_tree(&loc_name).unwrap();
            let locs_iter = locs.iter();
            let asm = None;
            Injection {
                arch: ctx.config.arch.clone(),
                mutant,
                asm,
                locs_iter,
                index: 0,
            }
        } else {
            panic!("Uninitialized db.")
        }
    }
}

impl Iterator for Injection {
    type Item = Change;

    fn next(&mut self) -> Option<Self::Item> {
        if self.asm.is_none() {
            // process new loc
            if let Some(Ok((_, value))) = self.locs_iter.next() {
                let asm = Asm::from(&value);
                let key = asm.key(&self.arch, self.index as u8);
                self.index += 1; // update index
                self.asm = Some(asm); // update cur asm
                if let Ok(Some(value)) = self.mutant.get(&key) {
                    let chg_asm = Asm::from(&value);
                    Some(Change(chg_asm))
                } else {
                    self.next()
                }
            } else {
                None
            }
        } else {
            let asm = self.asm.as_ref().unwrap();
            if self.index == asm.size * 8 {
                self.asm = None;
                self.index = 0;
                self.next()
            } else {
                let key = asm.key(&self.arch, self.index as u8);
                self.index += 1; // update index
                if let Ok(Some(value)) = self.mutant.get(&key) {
                    let chg_asm = Asm::from(&value);
                    Some(Change(chg_asm))
                } else {
                    self.next()
                }
            }
        }
    }
}
