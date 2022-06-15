use crate::asm::inst::*;

impl std::fmt::Display for Arg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Arg::Imm(n) => write!(f, "0x{:x}", n),
            Arg::Sym(s) | Arg::Reg(s) => write!(f, "{}", s),
            Arg::Mem(addr) => write!(f, "{}", addr),
            Arg::Expr(op, arg) => write!(f, "{} {}", op, arg),
        }
    }
}

impl std::fmt::Display for Addr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(len) = &self.len {
            write!(f, "{} ", len)?;
        }
        if let Some(sel) = &self.sel {
            write!(f, "{}:", sel)?;
        }
        write!(f, "[{}]", self.value)
    }
}

impl std::fmt::Display for RegShft {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.reg, &self.shft) {
            (Some(reg), Some(n)) => write!(f, "{} + {}", reg, n),
            (None, Some(n)) => write!(f, "{:x}", n),
            (Some(reg), None) => write!(f, "{}", reg),
            (None, None) => unreachable!(),
        }
    }
}

impl std::fmt::Display for Inst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.op)?;
        if let Some(args) = &self.args {
            let mut flag = false;
            for arg in args {
                if flag {
                    write!(f, ",")?;
                }
                write!(f, " {}", arg)?;
                flag = true;
            }
        }
        Ok(())
    }
}
