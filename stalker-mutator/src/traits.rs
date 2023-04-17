use stalker_utils::{asm::Asm, tag::Tag};
use std::{iter::Map, marker::PhantomData};

pub trait FaultModel
where
    Self: Sized + Tag + Sync + Send,
{
    fn next_mutant(iter: &mut IntoIter<Self>) -> Option<Vec<u8>>;
}

/// RawMutants is defined to implement value-based mutants, such as bytes or bits.
pub struct IntoIter<T: FaultModel> {
    pub phantom: PhantomData<T>,
    pub(crate) base: Vec<u8>,
    pub(crate) size: usize,
    pub(crate) bit: (usize, usize),
    pub(crate) mask: u8,
}

pub trait RawMutatable
where
    Self: Sized,
{
    fn raw_mutants<M: FaultModel>(&self) -> IntoIter<M>;
}

impl RawMutatable for Vec<u8> {
    fn raw_mutants<M: FaultModel>(&self) -> IntoIter<M> {
        IntoIter {
            phantom: PhantomData::default(),
            base: self.to_vec(),
            size: self.len(),
            bit: (0, 0),
            mask: 1,
        }
    }
}

impl<M: FaultModel> Iterator for IntoIter<M> {
    type Item = Vec<u8>;
    fn next(&mut self) -> Option<Self::Item> {
        M::next_mutant(self)
    }
}

pub trait Mutatable<T, F: FnMut(Vec<u8>) -> T> {
    fn mutants<M: FaultModel>(&self, map_f: F) -> Map<IntoIter<M>, F>;
}

impl<T, F: FnMut(Vec<u8>) -> T> Mutatable<T, F> for Asm {
    fn mutants<M: FaultModel>(&self, f: F) -> Map<IntoIter<M>, F> {
        self.bytes.raw_mutants::<M>().map(f)
    }
}
