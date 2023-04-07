pub mod asm;
pub mod binary;
pub mod config;
pub mod context;
mod db;
mod error;
pub mod fmt;
pub mod loc;

pub type Result<T, E = error::Error> = core::result::Result<T, E>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loc() -> anyhow::Result<()> {
        let ctx = context::PreContext::default()
            .data_path("/tmp/stalker")
            .init()?;
        let mut lib = ctx.lib;
        let mut rz = ctx.rz;
        lib.init_locs(&mut rz)?;
        let mut snames = vec![];
        for _ in 0..lib.locs.len() {
            snames.push(lib.locs[3].name.clone());
        }
        for sname in snames.iter() {
            let _locinfo = lib.get_locinfo(&mut rz, &sname)?;
        }
        Ok(())
    }

    #[test]
    fn test_parser() {
        let insts = vec![
            "jmp 0x4cd04",
            "cmp qword [rbp - 8], 0xc",
            "movzx eax, byte [rax]",
            "mov rbp, rsp",
            "call sym.imp.printf",
            "ret",
            "ret;",
            "sub rdx, qword fs:[rdi + 0x28]",
            "orr x1, xzr, x26, lsl 1",
            "ldr w8, 0xfffffffffff06a20",
        ];
        for i in insts {
            let ast = asm::AsmParser::parse(i).unwrap();
            println!("ASM: {}  ", i);
            println!("FMT: {}", ast);
            println!("AST: {:?}", ast);
            println!();
        }

        let insts = vec!["pop rax;test eax, eax;", "push rcx; and esi, 0xff0000"];

        for i in insts {
            let ast = asm::AsmParser::parse_many(i).unwrap();
            println!("ASM: {}  ", i);
            println!("AST: {:?}", ast);
            println!();
        }
    }

    #[test]
    fn test_diff() {
        let insts = vec![
            ("jmp 0x4cd04", "jmp 0x4cd01"),
            ("cmp qword [rbp - 0], 0xc", "cmp qword [rbp - 8], 0xc"),
            ("cmp qword [rbp - 8], 0xd", "cmp qword [rbp - 8], 0xc"),
            ("cmp qword [rbp - 8], 0xc", "cmp [rbp - 8], 0xc"),
            ("movzx eax, byte [rax]", "movzx eax, [rax]"),
            ("mov rbp, rsp", "mov rax, rsp"),
            ("call sym.imp.printf", "call 0x00"),
            ("ret", "nop"),
        ];
        for (a, b) in insts {
            let (ia, ib) = (
                asm::AsmParser::parse(a).unwrap(),
                asm::AsmParser::parse(b).unwrap(),
            );
            println!("{:?}", ia.diff(&ib));
        }
    }
}
