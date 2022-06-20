pub mod asm;
pub mod config;
pub mod context;
mod fmt;
pub mod loc;

#[cfg(test)]
mod tests {
    use super::*;
    use serfig::collectors::{from_env, from_file, from_self};
    use serfig::parsers::Toml;
    use serfig::Builder;

    #[test]
    fn test_loc() -> anyhow::Result<()> {
        let mut lib = loc::LibInstance::default();
        lib.init_locs()?;
        let mut snames = vec![];
        for _ in 0..lib.locs.len() {
            snames.push(lib.locs[3].name.clone());
        }
        for sname in snames.iter() {
            let _locinfo = lib.get_locinfo(&sname)?;
        }
        Ok(())
    }

    #[test]
    fn test_config() -> anyhow::Result<()> {
        let builder = Builder::default()
            .collect(from_env())
            .collect(from_file(Toml, "config.toml"))
            .collect(from_self(config::StalkerConfig::default()));
        let t: config::StalkerConfig = builder.build()?;

        println!("{:?}", t);
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
