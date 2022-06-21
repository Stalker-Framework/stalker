use anyhow::Result;
use crossterm::{
    cursor::{RestorePosition, SavePosition},
    execute,
    terminal::{Clear, ClearType},
};
use stalker_mutator::Mutatable;
use stalker_utils::asm::Asm;
use stalker_utils::context::Context;
use stalker_utils::fmt::hex;
use std::env;
use std::io::stdout;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut ctx = Context::new(args.get(1).unwrap_or(&String::from("/bin/ls")))?;

    ctx.lib.init_locs(&mut ctx.rz)?;
    ctx.init_db()?;
    if let Some(db) = ctx.db {
        let locs = ctx.lib.locs.to_vec();
        let cnt = locs.len();
        for (i, loc) in locs.iter().enumerate() {
            println!("{:2}/{:2} {}", i, cnt, &loc.name);
            let locinfo = ctx.lib.get_locinfo(&mut ctx.rz, &loc.name);
            if locinfo.is_err() {
                continue;
            }
            for op in locinfo.unwrap().ops.iter() {
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
                        execute!(stdout(), Clear(ClearType::CurrentLine), SavePosition)?;
                        print!("{} {}", &key, &val);
                        execute!(stdout(), RestorePosition)?;
                        db.mutant.insert(key, val.as_str())?;
                    }
                }
            }
        }
    } else {
        panic!("Db not initialized.");
    }
    Ok(())
}
