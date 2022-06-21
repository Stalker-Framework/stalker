use crate::asm::{
    inst::{Addr, Arg, Inst},
    Asm, AsmParser,
};
use crate::error::Error;
use crate::Result;

#[derive(Debug)]
pub enum InstDiff {
    Op(String, String),
    Arg(usize, Option<ArgDiff>),
    Same,
}

#[derive(Debug)]
pub enum ArgDiff {
    Imm,
    Reg,
    Mem(MemDiff),
    Type,
}

#[derive(Debug)]
pub enum MemDiff {
    Shift(Option<i64>, Option<i64>),
    Reg(Option<String>, Option<String>),
    Len(Option<String>, Option<String>),
}

impl Inst {
    pub fn diff(&self, b: &Self) -> InstDiff {
        // Check op
        if self.op != b.op {
            return InstDiff::Op(self.op.clone(), b.op.clone());
        }
        // Check args
        if let (Some(args_a), Some(args_b)) = (&self.args, &b.args) {
            let mut args_iter_a = args_a.iter();
            let mut args_iter_b = args_b.iter();
            let mut i = 0;
            loop {
                match (args_iter_a.next(), args_iter_b.next()) {
                    (None, None) => return InstDiff::Same,
                    (_, None) | (None, _) => return InstDiff::Arg(i, None),
                    (Some(arg_a), Some(arg_b)) => {
                        if arg_a == arg_b {
                            i += 1;
                            continue;
                        } else {
                            match (arg_a, arg_b) {
                                (Arg::Imm(_), Arg::Reg(_)) | (Arg::Reg(_), Arg::Imm(_)) => {
                                    return InstDiff::Arg(i, Some(ArgDiff::Type));
                                }
                                (Arg::Reg(_), Arg::Reg(_)) => {
                                    return InstDiff::Arg(i, Some(ArgDiff::Reg));
                                }
                                (Arg::Mem(aa), Arg::Mem(ab)) => {
                                    return InstDiff::Arg(i, Some(ArgDiff::Mem(aa.diff(ab))))
                                }
                                (_, _) => return InstDiff::Arg(i, Some(ArgDiff::Imm)),
                            }
                        }
                    }
                }
            }
        } else {
            InstDiff::Arg(0, None)
        }
    }
}

impl Addr {
    pub fn diff(&self, b: &Self) -> MemDiff {
        if self.len != b.len {
            return MemDiff::Len(self.len.clone(), b.len.clone());
        };
        if self.value.reg != b.value.reg {
            return MemDiff::Reg(self.value.reg.clone(), b.value.reg.clone());
        };
        if self.value.shft != b.value.shft {
            return MemDiff::Shift(self.value.shft, b.value.shft);
        };
        unreachable!()
    }
}

impl Asm {
    pub fn meta(&self) -> Result<Inst> {
        let disasm = self.disasm.as_ref().ok_or(Error::Disasm)?.to_string();
        AsmParser::parse(&disasm)
    }

    pub fn diff(&self, b: &Self) -> Result<InstDiff> {
        let meta_a = self.meta()?;
        let meta_b = b.meta()?;
        Ok(meta_a.diff(&meta_b))
    }
}
