mod config;
mod exec;
mod fmt;
use std::marker::PhantomData;

pub use config::InjectionConfig;
use sled::Tree;
use stalker_mutator::FaultModel;
use stalker_utils::{asm::Asm, config::Arch, context::Context, loc::LocAsm};

pub struct Injection<M: FaultModel> {
    phantom: PhantomData<M>,
    arch: Arch,
    mutant: Tree,
    loc_asm: Option<LocAsm>,
    locs_iter: sled::Iter,
    index: u8,
}

#[derive(Debug)]
pub struct Change(LocAsm, Asm, u8);

pub trait Injectable {
    fn inject<M : FaultModel>(&self, loc_name: &str) -> Injection<M>;
}

impl Injectable for Context {
    fn inject<M : FaultModel>(&self, loc_name: &str) -> Injection<M> {
        Injection::new(self, loc_name)
    }
}

impl<M: FaultModel> Injection<M> {
    fn new(ctx: &Context, loc_name: &str) -> Self {
        if let Some(db) = &ctx.db {
            let loc_name = loc_name.to_string();
            let mutant = db.mutant.open_tree(M::tag()).expect("Mutants db not found!").clone();
            let locs = db.instruction.open_tree(loc_name).unwrap();
            let locs_iter = locs.iter();
            let asm = None;
            Injection {
                phantom: PhantomData::default(),
                arch: ctx.config.arch.clone(),
                mutant,
                loc_asm: asm,
                locs_iter,
                index: 0,
            }
        } else {
            panic!("Uninitialized db.")
        }
    }

    fn increment(&mut self, asm: Option<Asm>, loc_asm: LocAsm) -> Option<Change> {
        let asm: Asm = asm.unwrap_or_else(|| loc_asm.clone().into());
        let key = asm.key(&self.arch, self.index);
        self.index += 1;
        if let Ok(Some(value)) = self.mutant.get(key) {
            let chg_asm = Asm::from(&value);
            Some(Change(loc_asm, chg_asm, self.index))
        } else {
            self.next()
        }
    }
}

impl<M: FaultModel> Iterator for Injection<M> {
    type Item = Change;

    fn next(&mut self) -> Option<Self::Item> {
        if self.loc_asm.is_none() {
            // process new loc
            if let Some(Ok((offset, value))) = self.locs_iter.next() {
                // update current loc
                let asm = Asm::from(&value);
                let loc_asm = LocAsm::from((&offset, asm.clone()));
                self.loc_asm = Some(loc_asm.clone());
                self.increment(Some(asm), loc_asm)
            } else {
                // no new loc, end of the iteration
                None
            }
        } else {
            // process current loc
            let loc_asm = self.loc_asm.clone().unwrap();
            if self.index == loc_asm.size * 8 {
                // no more mutants, reset the index and prepare for a new loc
                self.loc_asm = None;
                self.index = 0;
                self.next()
            } else {
                // more mutants
                self.increment(None, loc_asm)
            }
        }
    }
}
