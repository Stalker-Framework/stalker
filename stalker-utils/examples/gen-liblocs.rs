use anyhow::Result;
use crossterm::{
    cursor::{RestorePosition, SavePosition},
    execute,
    terminal::{Clear, ClearType},
};
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
            let tree = db.instruction.open_tree(&loc.name)?;
            if locinfo.is_err() {
                continue;
            }
            for asm in locinfo.unwrap().ops.iter() {
                if let Ok(Some(_)) = tree.get(format!("{:08x}", asm.offset)) {
                    continue;
                } else {
                    let m = asm;
                    let key = format!("{:08x}", asm.offset);
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
                    tree.insert(key, val.as_str())?;
                }
            }
        }
    } else {
        panic!("Db not initialized.");
    }
    Ok(())
}
