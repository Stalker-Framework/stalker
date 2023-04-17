use stalker_mutator::FaultModel;
use stalker_utils::asm::Asm;

use crate::{Change, Injection};

impl<M : FaultModel> std::fmt::Display for Injection<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Source: ")?;
        if let Some(asm) = &self.loc_asm {
            let asm: Asm = asm.into();
            write!(f, "{}", asm)?;
        } else {
            write!(f, "(Uninitialized)")?;
        }
        Ok(())
    }
}

impl std::fmt::Display for Change {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Change: {{ offset: {:08x} }}", self.0.offset)?;
        let asm: Asm = (&self.0).into();
        writeln!(f, "  from: {}", asm)?;
        write!(f, "    to: {}", self.1)?;
        Ok(())
    }
}
