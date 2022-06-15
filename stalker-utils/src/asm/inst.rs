#[derive(Debug)]
pub struct Inst {
    pub op: String,
    pub args: Option<Vec<Arg>>,
}

#[derive(Debug)]
pub enum Arg {
    Imm(i64),
    Sym(String),
    Reg(String),
    Mem(Addr),
    Expr(String, String),
}

#[derive(Debug)]
pub struct RegShft {
    pub reg: Option<String>,
    pub shft: Option<i64>,
}

#[derive(Debug)]
pub struct Addr {
    pub len: Option<String>,
    pub sel: Option<String>,
    pub value: RegShft,
}
